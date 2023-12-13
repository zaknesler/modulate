use crate::db::error::DbError;
use chrono::{DateTime, Utc};
use rusqlite::Row;

pub const COLUMNS: &str = "id, watcher_id, num_tracks_transferred,error, synced_at, created_at";

#[derive(Debug, Clone)]
pub struct Transfer {
    pub id: u32,
    pub watcher_id: u32,
    pub num_tracks_transferred: u32,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<&Row<'_>> for Transfer {
    type Error = DbError;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get(0)?,
            watcher_id: row.get(1)?,
            num_tracks_transferred: row.get(2)?,
            error: row.get(3)?,
            created_at: row.get::<_, String>(4)?.parse()?,
        })
    }
}
