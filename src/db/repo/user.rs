use crate::{
    api::token::Token,
    db::{
        error::{DbError, DbResult},
        model::user::{User, COLUMNS},
    },
};
use rusqlite::params;

pub struct UserRepo {
    ctx: crate::context::AppContext,
}

impl UserRepo {
    pub fn new(ctx: crate::context::AppContext) -> Self {
        Self { ctx }
    }

    /// Create a new user record with a token or overwrite an existing user's token.
    pub fn upsert_user_token(&self, user_uri: &str, token: &Token) -> DbResult<User> {
        self.ctx
            .db
            .get()?
            .prepare(
                &format!("INSERT OR REPLACE INTO users (user_uri, token, created_at) VALUES (?1, ?2, ?3) RETURNING {COLUMNS}"),
            )?
            .query_and_then(
                params![
                    user_uri,
                    serde_json::to_string(token)?,
                    chrono::Utc::now().to_rfc3339()
                ],
                |row| User::try_from(row)
            )?
            .collect::<DbResult<Vec<_>>>()?
            .first()
            .cloned()
            .ok_or_else(|| DbError::SQLiteError(rusqlite::Error::QueryReturnedNoRows))
    }

    /// Try to find a user's auth token.
    pub fn find_user_by_uri(&self, user_uri: &str) -> DbResult<Option<User>> {
        Ok(self
            .ctx
            .db
            .get()?
            .prepare(format!("SELECT {COLUMNS} FROM users WHERE user_uri = ?1 LIMIT 1").as_ref())?
            .query_and_then(params![user_uri], |row| User::try_from(row))?
            .collect::<DbResult<Vec<_>>>()?
            .first()
            .cloned())
    }

    /// Delete a user by their Spotify URI
    pub fn delete_user_by_uri(&self, user_uri: &str) -> DbResult<()> {
        self.ctx
            .db
            .get()?
            .prepare("DELETE FROM users WHERE user_uri = ?1")?
            .execute(params![user_uri])
            .map(|_| ())
            .map_err(|err| err.into())
    }
}
