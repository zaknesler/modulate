pub type BaseResult<T> = Result<T, BaseError>;

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum BaseError {
    #[error(transparent)]
    ConfigError(#[from] figment::Error),

    #[error(transparent)]
    DotEnvError(#[from] dotenvy::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ClientError(#[from] crate::api::error::ClientError),

    #[error(transparent)]
    SyncError(#[from] crate::sync::error::SyncError),

    #[error(transparent)]
    DbError(#[from] crate::db::error::DbError),

    #[error(transparent)]
    WebError(#[from] crate::web::error::WebError),
}
