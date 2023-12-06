use rusqlite::params;

use crate::api::token::Token;

pub struct UserRepo {
    ctx: crate::context::AppContext,
}

impl UserRepo {
    pub fn new(ctx: crate::context::AppContext) -> Self {
        Self { ctx }
    }

    /// Create a new user record with a token or overwrite an existing user's token.
    pub fn upsert_user_token(&self, user_id: &str, token: &Token) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare(
                "INSERT OR REPLACE INTO users (user_id, token, created_at) VALUES (?1, ?2, ?3)",
            )?
            .execute(params![
                user_id,
                serde_json::to_string(token)?,
                chrono::Utc::now().to_rfc3339()
            ])?;

        Ok(())
    }

    /// Try to find a user's auth token.
    pub fn get_token_by_user_id(&self, user_id: &str) -> crate::Result<String> {
        self.ctx
            .db
            .get()?
            .prepare("SELECT token FROM users WHERE user_id = ?1 LIMIT 1")?
            .query_row(params![user_id], |row| Ok(row.get(0)?))
            .map_err(|err| err.into())
    }

    /// Delete a user by ID.
    pub fn delete_user_by_id(&self, user_id: &str) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare("DELETE FROM users WHERE user_id = ?1")?
            .execute(params![user_id])
            .map(|_| ())
            .map_err(|err| err.into())
    }
}
