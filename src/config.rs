use serde::Deserialize;
use std::path;

pub const CONFIG_DIR: &str = ".config";
const CONFIG_ENV_PREFIX: &str = "MODULATE";
const CONFIG_FILE_PRECEDENCE: [&str; 2] = ["default.toml", "local.toml"];

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub log_level: LogLevel,
    pub sync: SyncConfig,
    pub db: DbConfig,
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
pub struct SyncConfig {
    pub enabled: bool,
    pub interval_mins: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DbConfig {
    pub file: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub allowed_origins: Vec<String>,
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
    pub callback_uri: String,
}

impl Config {
    pub fn try_parse() -> crate::Result<Config> {
        let dir = path::Path::new(CONFIG_DIR);

        Ok(CONFIG_FILE_PRECEDENCE
            .iter()
            .fold(::config::Config::builder(), |config, file| {
                config.add_source(::config::File::with_name(dir.join(file).to_str().unwrap()))
            })
            .add_source(
                ::config::Environment::with_prefix(CONFIG_ENV_PREFIX)
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" "),
            )
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
