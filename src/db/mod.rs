use self::error::DbResult;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub mod error;
pub mod model;
pub mod repo;

pub fn init(db_path: &str) -> DbResult<Pool<SqliteConnectionManager>> {
    let db_manager = SqliteConnectionManager::file(db_path);
    let db = Pool::new(db_manager)?;

    let conn = db.get()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            user_uri     TEXT    UNIQUE,
            token       TEXT    NOT NULL,
            created_at  TEXT    NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS watchers (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            user_uri         TEXT    NOT NULL,
            playlist_from   TEXT    NOT NULL,
            playlist_to     TEXT    NOT NULL,
            should_remove   BOOLEAN CHECK (should_remove IN (0, 1)),
            sync_interval   TEXT    NOT NULL,
            next_sync_at    TEXT,
            created_at      TEXT    NOT NULL,

            UNIQUE (user_uri, playlist_from, playlist_to)
        )",
        [],
    )?;

    Ok(db)
}
