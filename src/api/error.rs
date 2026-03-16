use oauth2::{RequestTokenError, basic::BasicErrorResponse};

pub type ClientResult<T> = Result<T, ClientError>;

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("invalid ID: {0}")]
    InvalidId(String),

    #[error("spotify {status} error: {message}")]
    ApiError { status: u16, message: String },

    #[error("missing refresh token")]
    MissingRefreshToken,

    #[error("mutex lock error")]
    MutexLockError,

    #[error("spotify did not return scopes")]
    SpotifyDidNotReturnScopes,

    #[error("spotify did not return expires_in")]
    SpotifyDidNotReturnExpiresIn,

    #[error("too many requests")]
    TooManyRequests,

    #[error("spotify returned an empty response")]
    EmptyResponse,

    #[error(transparent)]
    ChronoOutOfRangeError(#[from] chrono::OutOfRangeError),

    #[error(transparent)]
    OAuthParseError(#[from] oauth2::url::ParseError),

    #[error(transparent)]
    OAuthRequestError(#[from] oauth2::reqwest::Error),

    #[error(transparent)]
    OAuthTokenError(
        #[from]
        RequestTokenError<oauth2::HttpClientError<oauth2::reqwest::Error>, BasicErrorResponse>,
    ),

    #[error(transparent)]
    RegexError(#[from] regex::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    HeaderValueError(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    DbError(#[from] crate::db::error::DbError),
}
