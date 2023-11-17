use thiserror::Error;

pub type SpotifyResult<T> = Result<T, SpotifyError>;

#[derive(Error, Debug)]
pub enum SpotifyError {
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    SpotifyClientError(#[from] rspotify::ClientError),

    #[error(transparent)]
    SpotifyIdError(#[from] rspotify::model::IdError),
}
