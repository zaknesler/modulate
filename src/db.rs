use crate::config::CONFIG_DIR;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::path;

pub fn init_db(file: String) -> crate::Result<Pool<SqliteConnectionManager>> {
    let db_path = path::Path::new(CONFIG_DIR).join(file);
    let db_manager = r2d2_sqlite::SqliteConnectionManager::file(db_path);
    let db = r2d2::Pool::new(db_manager)?;

    // Ensure db table exists
    db.get()?.execute(
        "CREATE TABLE IF NOT EXISTS tokens (token VARCHAR)",
        params![],
    )?;

    Ok(db)
}
