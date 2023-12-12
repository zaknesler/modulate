use crate::{api::token::Token, db::error::DbError};
use chrono::{DateTime, Utc};
use rusqlite::Row;

pub const COLUMNS: &str = "id, user_uri, token, created_at";

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub user_uri: String,
    pub token: Token,
    pub synced_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<&Row<'_>> for User {
    type Error = DbError;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get(0)?,
            user_uri: row.get(1)?,
            token: serde_json::from_str(&row.get::<_, String>(2)?)?,
            synced_at: row.get::<_, String>(3)?.parse()?,
            created_at: row.get::<_, String>(3)?.parse()?,
        })
    }
}
