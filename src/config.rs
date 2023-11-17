use std::path;

use serde::Deserialize;

use crate::error::SpotifyResult;

const CONFIG_DIR: &str = ".config";
const CONFIG_ENV_PREFIX: &str = "SPOTIFY";
const CONFIG_FILE_PRECEDENCE: [&str; 2] = ["default.toml", "local.toml"];

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub log_level: LogLevel,
    pub web: WebConfig,
    pub spotify: SpotifyConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
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

impl From<LogLevel> for tracing::metadata::LevelFilter {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Trace => tracing::metadata::LevelFilter::TRACE,
            LogLevel::Debug => tracing::metadata::LevelFilter::DEBUG,
            LogLevel::Info => tracing::metadata::LevelFilter::INFO,
            LogLevel::Warn => tracing::metadata::LevelFilter::WARN,
            LogLevel::Error => tracing::metadata::LevelFilter::ERROR,
        }
    }
}
