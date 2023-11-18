use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error("config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("unauthorized")]
    UnauthorizedError,

    #[error("database error: {0}")]
    DbError(#[from] r2d2::Error),

    #[error("json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("sqlite error: {0}")]
    SQLiteError(#[from] r2d2_sqlite::rusqlite::Error),

    #[error("spotify client error: {0}")]
    SpotifyClientError(#[from] rspotify::ClientError),

    #[error("spotify ID error: {0}")]
    SpotifyIdError(#[from] rspotify::model::IdError),

    #[error("address parse error: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::UnauthorizedError => (
                StatusCode::UNAUTHORIZED,
                Value::String(Self::UnauthorizedError.to_string()),
            ),
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
