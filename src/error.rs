use thiserror::Error;

pub type SpotifyResult<T> = Result<T, SpotifyError>;

#[derive(Error, Debug)]
pub enum SpotifyError {
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
}
