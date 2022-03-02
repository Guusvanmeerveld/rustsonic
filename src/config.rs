use crate::keyring;

use std::{env, fs};

use rpassword;

use serde_derive::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
pub struct SubsonicConfig {
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub subsonic: SubsonicConfig,
    #[serde(default = "daemon_default")]
    pub daemon: bool,
}

fn daemon_default() -> bool {
    false
}

impl Default for Config {
    fn default() -> Config {
        Config {
            daemon: daemon_default(),
            subsonic: SubsonicConfig {
                url: None,
                username: None,
                password: None,
            },
        }
    }
}

pub fn read_config(config_locations: Vec<String>) -> Config {
    let mut i = 0;

    let contents = loop {
        if i >= config_locations.len() {
            break String::new();
        }

        break match fs::read_to_string(&config_locations[i]) {
            Ok(value) => value,
            Err(_) => {
                i += 1;
                continue;
            }
        };
    };

    let mut config: Config = if !contents.is_empty() {
        toml::from_str(&contents).unwrap()
    } else {
        Config::default()
    };

    let args: Vec<String> = env::args().collect::<Vec<String>>().clone();

    let keyring_password = keyring::get_password(config.subsonic.username.as_ref().unwrap()).ok();

    let mut i = 1;
    while i < args.len() {
        let next_item = if i + 1 < args.len() {
            Option::from(args[i + 1].clone())
        } else {
            None
        };

        match &*args[i] {
            "--username" | "-u" => {
                config.subsonic.username = next_item;
            }
            "--url" | "-U" => {
                config.subsonic.url = next_item;
            }
            "--password" | "-p" => {
                let password = rpassword::read_password_from_tty(Some("Subsonic password: "));

                if keyring_password == None {
                    keyring::set_password(
                        config.subsonic.username.as_ref().unwrap(),
                        password.as_ref().unwrap(),
                    );
                }

                config.subsonic.password = password.ok();
            }
            "--daemon" | "-D" => config.daemon = true,
            _ => {}
        }

        i += 1;
    }

    if (config.subsonic.password == None) & (config.subsonic.username != None) {
        config.subsonic.password = keyring_password;
    }

    if (config.subsonic.url == None)
        | (config.subsonic.username == None)
        | (config.subsonic.password == None)
    {
        panic!("Missing url, username or password")
    }

    config
}
