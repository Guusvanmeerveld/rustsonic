use crate::config::Config;
use crate::constants;
use crate::utils;

use hyper::{body::Buf, client::HttpConnector, Client, Uri};
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
        let response = self.make_request("ping.view").await;

        match response {
            Ok(value) => value.status == ApiStatus::Ok,
            Err(_) => false,
        }
    }

    async fn make_request(&self, location: &str) -> utils::Result<SubsonicResponse> {
        let username = self
            .config
            .subsonic
            .username
            .as_ref()
            .expect("No username was provided");

        // Create uri
        let user_uri: Uri = self
            .config
            .subsonic
            .url
            .as_ref()
            .expect("No url provided")
            .parse::<Uri>()
            .expect("Parsing the user provided uri");

        // Create salt
        let salt = utils::random_string(8);

        let password = self.config.subsonic.password.as_ref().unwrap();

        let salt_and_password = format!("{}{}", password, salt);

        let token = format!("{:x}", md5::compute(salt_and_password));

        let query_params = querystring::stringify(vec![
            ("v", constants::API_VERSION),
            ("c", constants::APPLICATION_NAME),
            ("u", &username),
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

        let connector = HttpConnector::new();

        let client = Client::builder().build::<_, hyper::Body>(connector);

        let request = client.get(uri);

        let response: hyper::Response<hyper::body::Body> =
            request.await.expect("Sending the request");

        let body = hyper::body::aggregate(response)
            .await
            .expect("Aggregating the response");

        let xml: SubsonicResponse = serde_xml_rs::from_reader(body.reader())?;

        Ok(xml)
    }
}
