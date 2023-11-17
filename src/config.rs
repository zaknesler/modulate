use std::path;

use serde::Deserialize;

use crate::error::SpotifyResult;

const CONFIG_DIR: &str = ".config";
const CONFIG_ENV_PREFIX: &str = "SPOTIFY";
const CONFIG_FILE_PRECEDENCE: [&str; 2] = ["default.toml", "local.toml"];

#[derive(Debug, Deserialize)]
pub struct Config {
    pub debug: bool,
    pub web: WebConfig,
    pub spotify: SpotifyConfig,
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl Config {
    pub fn try_parse() -> SpotifyResult<Config> {
        let dir = path::Path::new(CONFIG_DIR);

        Ok(CONFIG_FILE_PRECEDENCE
            .iter()
            .fold(::config::Config::builder(), |config, file| {
                config.add_source(::config::File::with_name(dir.join(file).to_str().unwrap()))
            })
            .add_source(::config::Environment::with_prefix(CONFIG_ENV_PREFIX))
            .build()?
            .try_deserialize()?)
    }
}
