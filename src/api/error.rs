pub type ClientResult<T> = Result<T, ClientError>;

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("invalid ID: {0}")]
    InvalidId(String),

    #[error("spotify {status} error: {message}")]
    ApiError { status: u16, message: String },

    #[error("missing access token")]
    MissingAccessToken,

    #[error("missing refresh token")]
    MissingRefreshToken,

    #[error("missing token")]
    MissingToken,

    #[error("mutex lock error")]
    MutexLockError,

    #[error("spotify did not return scopes")]
    SpotifyDidNotReturnScopes,

    #[error("spotify did not return expires_in")]
    SpotifyDidNotReturnExpiresIn,

    #[error(transparent)]
    ChronoOutOfRangeError(#[from] chrono::OutOfRangeError),

    #[error(transparent)]
    OAuthParseError(#[from] oauth2::url::ParseError),

    #[error(transparent)]
    RegexError(#[from] regex::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    HeaderValueError(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error(transparent)]
    DbError(#[from] crate::db::error::DbError),
}
