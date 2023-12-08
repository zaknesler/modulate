pub type DbResult<T> = Result<T, DbError>;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("invalid ID: {0}")]
    InvalidId(String),

    #[error("invalid sync interval: {0}")]
    InvalidSyncInterval(String),

    #[error(transparent)]
    DateParseError(#[from] chrono::ParseError),

    #[error(transparent)]
    PoolError(#[from] r2d2::Error),

    #[error(transparent)]
    SQLiteError(#[from] r2d2_sqlite::rusqlite::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}
