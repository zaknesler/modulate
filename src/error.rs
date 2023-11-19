use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unauthorized")]
    UnauthorizedError,

    #[error("jwt expired")]
    JwtExpiredError,

    #[error("invalid jwt")]
    JwtInvalidError,

    #[error("config error: {0:?}")]
    ConfigError(#[from] config::ConfigError),

    #[error("database error: {0:?}")]
    DbError(#[from] r2d2::Error),

    #[error("json error: {0:?}")]
    JsonError(#[from] serde_json::Error),

    #[error("sqlite error: {0:?}")]
    SQLiteError(#[from] r2d2_sqlite::rusqlite::Error),

    #[error("spotify client error: {0:?}")]
    SpotifyClientError(#[from] rspotify::ClientError),

    #[error("spotify ID error: {0:?}")]
    SpotifyIdError(#[from] rspotify::model::IdError),

    #[error("addr parse error: {0:?}")]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error("hmac error: {0:?}")]
    HmacError(#[from] hmac::digest::InvalidLength),

    #[error("jwt error: {0:?}")]
    JwtError(#[from] jwt::Error),

    #[error("chrono parse error: {0:?}")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("validation errors: {0:?}")]
    ValidationErrors(#[from] validator::ValidationErrors),

    #[error("validation error: {0:?}")]
    ValidationError(#[from] validator::ValidationError),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::UnauthorizedError | Self::JwtExpiredError | Self::JwtInvalidError => {
                (StatusCode::UNAUTHORIZED, Value::String(self.to_string()))
            }
            Self::ValidationErrors(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                json!({ "fields": err.field_errors() }),
            ),
            Self::ValidationError(err) => (StatusCode::UNPROCESSABLE_ENTITY, json!(err)),
            _ => {
                tracing::error!("{:?}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Value::String("unexpected error occurred".into()),
                )
            }
        };

        let data = Json(json!({ "status": status.as_u16(), "error": error, }));
        (status, data).into_response()
    }
}
