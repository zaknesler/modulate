use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Debug, Clone)]
pub struct ApiContext {
    pub config: crate::config::Config,
    pub db: Pool<SqliteConnectionManager>,
}
