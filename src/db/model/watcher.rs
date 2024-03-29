use super::playlist::PlaylistType;
use crate::db::error::DbError;
use chrono::{DateTime, Utc};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

pub const COLUMNS: &str = "id, user_uri, playlist_from, playlist_to, should_remove, sync_interval, last_sync_at, next_sync_at, created_at";

#[derive(Debug, Clone)]
pub struct Watcher {
    pub id: u32,
    pub user_uri: String,
    pub playlist_from: PlaylistType,
    pub playlist_to: PlaylistType,
    pub should_remove: bool,
    pub sync_interval: SyncInterval,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub next_sync_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<&Row<'_>> for Watcher {
    type Error = DbError;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get(0)?,
            user_uri: row.get(1)?,
            playlist_from: PlaylistType::try_from_value(&row.get::<_, String>(2)?)?,
            playlist_to: PlaylistType::try_from_value(&row.get::<_, String>(3)?)?,
            should_remove: row.get(4)?,
            sync_interval: row.get::<_, String>(5)?.parse()?,
            last_sync_at: row.get::<_, Option<String>>(6)?.and_then(|val| val.parse().ok()),
            next_sync_at: row.get::<_, Option<String>>(7)?.and_then(|val| val.parse().ok()),
            created_at: row.get::<_, String>(8)?.parse()?,
        })
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SyncInterval {
    #[default]
    Hour,
    Day,
    Week,
}

impl Display for SyncInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Hour => write!(f, "hour"),
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
        }
    }
}

impl FromStr for SyncInterval {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "hour" => SyncInterval::Hour,
            "day" => SyncInterval::Day,
            "week" => SyncInterval::Week,
            _ => return Err(DbError::InvalidSyncInterval(s.to_string())),
        })
    }
}

impl From<SyncInterval> for chrono::Duration {
    fn from(value: SyncInterval) -> Self {
        match value {
            SyncInterval::Hour => chrono::Duration::try_hours(1).expect("won't overflow"),
            SyncInterval::Day => chrono::Duration::try_days(1).expect("won't overflow"),
            SyncInterval::Week => chrono::Duration::try_weeks(1).expect("won't overflow"),
        }
    }
}
