use crate::config::Config;
use crate::constants;
use crate::keyring;
use crate::utils;

use hyper::{body::Buf, Client, Uri};
use hyper_tls::HttpsConnector;

use querystring;

use md5;

use serde_derive::Deserialize;
use serde_xml_rs;

pub struct Api<'a> {
    pub config: &'a Config,
}

#[derive(Deserialize, PartialEq)]
struct SubsonicResponse {
    status: ApiStatus,
    version: String,
    // #[serde(rename = "$value")]
    // contents: Vec<Content>,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Content {
    Error(Error),
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum ApiStatus {
    Failed,
    Ok,
}

#[derive(Deserialize, PartialEq)]
struct Error {
    code: u8,
    message: String,
}

impl Api<'_> {
    /**
     * Checks if credentials and api version are compatible
     */
    pub async fn ping(&self) -> bool {
        let response: SubsonicResponse = self
            .make_request("ping.view")
            .await
            .expect("Ping remote server");

        response.status == ApiStatus::Ok
    }

    async fn make_request(&self, location: &str) -> utils::Result<SubsonicResponse> {
        // Create uri
        let user_uri: Uri = self
            .config
            .subsonic
            .url
            .parse::<Uri>()
            .expect("Parsing the user provided uri");

        // Create salt
        let salt = utils::random_string(8);

        let password = keyring::get_password(&self.config.subsonic.username);

        let password = match password {
            Ok(password) => password,
            Err(_) => self
                .config
                .subsonic
                .password
                .clone()
                .expect("Read password from config"),
        };

        let salt_and_password = format!("{}{}", password, salt);

        let token = format!("{:x}", md5::compute(salt_and_password));

        let query_params = querystring::stringify(vec![
            ("v", constants::API_VERSION),
            ("c", constants::APPLICATION_NAME),
            ("u", &self.config.subsonic.username),
            ("s", &salt),
            ("t", &token),
        ]);

        let scheme = user_uri.scheme().expect("Parse scheme from uri");

        let uri = format!(
            "{}://{}/rest/{}?{}",
            scheme,
            user_uri.authority().expect("Parse authority from uri"),
            location,
            query_params
        )
        .parse::<Uri>()
        .expect("Parsing the uri");

        // Http request
        let https = HttpsConnector::new();

        let client = Client::builder().build::<_, hyper::Body>(https);

        let request = client.get(uri);

        let response: hyper::Response<hyper::body::Body> =
            request.await.expect("Sending the request");

        let body = hyper::body::aggregate(response)
            .await
            .expect("Aggregating the response");

        let xml: SubsonicResponse =
            serde_xml_rs::from_reader(body.reader()).expect("Parsing the xml response");

        Ok(xml)
    }
}
