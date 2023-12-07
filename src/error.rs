use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid transfer: {0}")]
    InvalidTransfer(String),

    #[error("invalid form data: {0}")]
    InvalidFormData(String),

    #[error("resource not found")]
    NotFoundError,

    #[error("unauthorized")]
    UnauthorizedError,

    #[error("jwt expired")]
    JwtExpiredError,

    #[error("invalid jwt")]
    JwtInvalidError,

    #[error("mutex lock error")]
    MutexLockError,

    #[error(transparent)]
    OAuthUrlParseError(#[from] oauth2::url::ParseError),

    #[error(transparent)]
    ChronoOutOfRangeError(#[from] chrono::OutOfRangeError),

    #[error("invalid sync interval: {0}")]
    InvalidSyncInterval(String),

    #[error("invalid spotify ID: {0}")]
    InvalidSpotifyId(String),

    #[error("could not remove tracks from playlist: {0}")]
    CouldNotRemoveTracks(String),

    #[error("config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("database error: {0}")]
    DbError(#[from] r2d2::Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("sqlite error: {0}")]
    SQLiteError(#[from] r2d2_sqlite::rusqlite::Error),

    #[error("addr parse error: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error("hmac error: {0}")]
    HmacError(#[from] hmac::digest::InvalidLength),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt::Error),

    #[error("chrono parse error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("validation error: {0}")]
    ValidationError(#[from] validator::ValidationError),

    #[error("validation errors: {0}")]
    ValidationErrors(#[from] validator::ValidationErrors),

    #[error("invalid header value: {0}")]
    InvalidHeaderValueError(#[from] axum::http::header::InvalidHeaderValue),

    #[error("i/o error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    HeaderValueError(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::NotFoundError => (StatusCode::NOT_FOUND, Value::String(self.to_string())),
            Self::SQLiteError(_err @ rusqlite::Error::QueryReturnedNoRows) => (
                StatusCode::NOT_FOUND,
                Value::String(Self::NotFoundError.to_string()),
            ),
            Self::UnauthorizedError | Self::JwtExpiredError | Self::JwtInvalidError => {
                (StatusCode::UNAUTHORIZED, Value::String(self.to_string()))
            }
            Self::InvalidFormData(err) => (StatusCode::UNPROCESSABLE_ENTITY, Value::String(err)),
            Self::ValidationErrors(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                json!({ "fields": err.field_errors() }),
            ),
            Self::ValidationError(err) => (StatusCode::UNPROCESSABLE_ENTITY, json!(err)),
            _ => {
                tracing::error!("{:?}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Value::String(self.to_string()),
                )
            }
        };

        let data = Json(json!({ "status": status.as_u16(), "error": error }));
        (status, data).into_response()
    }
}
