use self::error::DbResult;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::{fs::File, path};

pub mod error;
pub mod model;
pub mod repo;

pub fn init(db_path: &str) -> DbResult<Pool<SqliteConnectionManager>> {
    // Create database file if it doesn't already exist
    if !path::Path::new(db_path).try_exists()? {
        File::create(db_path)?;
    }

    let db_manager = SqliteConnectionManager::file(db_path);
    let db = Pool::new(db_manager)?;

    let conn = db.get()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            user_uri    TEXT    NOT NULL UNIQUE,
            token       TEXT    NOT NULL,
            created_at  TEXT    NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS watchers (
            id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            user_uri        TEXT    NOT NULL,
            playlist_from   TEXT    NOT NULL,
            playlist_to     TEXT    NOT NULL,
            should_remove   BOOLEAN NOT NULL CHECK (should_remove IN (0, 1)),
            sync_interval   TEXT    NOT NULL,
            last_sync_at    TEXT,
            next_sync_at    TEXT,
            created_at      TEXT    NOT NULL,

            UNIQUE (user_uri, playlist_from, playlist_to)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transfers (
            id                      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            watcher_id              INTEGER NOT NULL,
            num_tracks_transferred  INTEGER NOT NULL,
            error                   TEXT    NOT NULL,
            synced_at               TEXT    NOT NULL,
            created_at              TEXT    NOT NULL
        )",
        [],
    )?;

    Ok(db)
}
