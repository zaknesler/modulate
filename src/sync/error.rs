pub type SyncResult<T> = Result<T, SyncError>;

#[derive(thiserror::Error, Debug)]
pub enum SyncError {
    #[error("invalid transfer: {0}")]
    InvalidTransferError(String),

    #[error("invalid sync interval: {0}")]
    InvalidSyncInterval(String),

    #[error("could not remove tracks from playlist: {0}")]
    CouldNotRemoveTracksError(String),

    #[error("unsupported transfer")]
    UnsupportedTransferError,

    #[error(transparent)]
    DbError(#[from] crate::db::error::DbError),

    #[error(transparent)]
    ClientError(#[from] crate::api::error::ClientError),
}
