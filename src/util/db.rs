use crate::config::CONFIG_DIR;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;

pub fn init_db(file: &str) -> crate::Result<Pool<SqliteConnectionManager>> {
    let db_path = Path::new(CONFIG_DIR).join(file);
    let db_manager = SqliteConnectionManager::file(db_path);
    let db = Pool::new(db_manager)?;

    // Ensure tables exist
    let conn = db.get()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (user_id VARCHAR(255) PRIMARY KEY, token VARCHAR(255) NOT NULL)",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS watchers (user_id VARCHAR(255) PRIMARY KEY, playlist_id VARCHAR(255))",
        [],
    )?;

    Ok(db)
}
