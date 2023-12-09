pub type WebResult<T> = Result<T, WebError>;

#[derive(thiserror::Error, Debug)]
pub enum WebError {
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

    #[error("invalid csrf")]
    CsrfInvalidError,

    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),

    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error(transparent)]
    HmacError(#[from] hmac::digest::InvalidLength),

    #[error(transparent)]
    JwtError(#[from] jwt::Error),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationError),

    #[error(transparent)]
    ValidationErrors(#[from] validator::ValidationErrors),

    #[error(transparent)]
    InvalidHeaderValueError(#[from] axum::http::header::InvalidHeaderValue),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ClientError(#[from] crate::api::error::ClientError),

    #[error(transparent)]
    SyncError(#[from] crate::sync::error::SyncError),

    #[error(transparent)]
    DbError(#[from] crate::db::error::DbError),
}
