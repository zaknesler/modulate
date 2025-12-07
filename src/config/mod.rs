use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

mod util;
pub use util::*;

pub mod error;
pub use error::*;

const ENV_CONFIG_HOME_PATH: &str = "MODULATE_HOME";
const ENV_PREFIX: &str = "MODULATE";
const PROJECT_DIR: &str = "modulate";
const DEFAULT_STUB: &str = "default.toml";
const LOCAL_CONFIG_FILE: &str = "config.toml";

#[derive(RustEmbed, Clone)]
#[folder = "$CARGO_MANIFEST_DIR/stubs"]
struct ConfigStubs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModulateConfig {
    pub log: LogConfig,
    pub sync: SyncConfig,
    pub database: DbConfig,
    pub web: WebConfig,
    pub spotify: SpotifyConfig,
    pub sentry: SentryConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogConfig {
    pub level: LogLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncConfig {
    pub enabled: bool,
    pub check_interval_mins: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbConfig {
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub public_url: String,
    pub jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SentryConfig {
    pub dsn: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, clap::ValueEnum)]
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
