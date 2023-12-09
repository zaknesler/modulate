pub type SyncResult<T> = Result<T, SyncError>;

#[derive(thiserror::Error, Debug)]
pub enum SyncError {
    #[error("invalid transfer: {0}")]
    InvalidTransferError(String),

    #[error("could not find user: {0}")]
    UserNotFoundError(String),

    #[error(transparent)]
    DbError(#[from] crate::db::error::DbError),

    #[error(transparent)]
    ClientError(#[from] crate::api::error::ClientError),
}
