use std::fs;

use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct SubsonicConfig {
    pub url: String,
    pub username: String,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub subsonic: SubsonicConfig,
}

pub fn read_config(config_locations: Vec<&str>) -> Config {
    let mut i = 0;

    let contents = loop {
        if config_locations.len() <= i {
            panic!("Failed to find any readable config file")
        }

        break match fs::read_to_string(config_locations[i]) {
            Ok(value) => value,
            Err(_) => {
                i += 1;
                continue;
            }
        };
    };

    let config: Config = toml::from_str(&contents).expect("Parse toml config");

    config
}
