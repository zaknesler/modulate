use crate::error::BaseResult;
use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub log: LogConfig,
    pub sync: SyncConfig,
    pub database: DbConfig,
    pub web: WebConfig,
    pub spotify: SpotifyConfig,
    pub sentry: SentryConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub level: LogLevel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SyncConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DbConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub public_url: String,
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SentryConfig {
    pub dsn: String,
}

impl Config {
    pub fn try_parse() -> BaseResult<Config> {
        let config = Figment::new()
            .merge(Env::raw().map(|key| key.as_str().to_lowercase().replacen("_", ".", 1).into()))
            .extract()?;

        Ok(config)
    }
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
