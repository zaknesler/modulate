pub struct UserRepo {
    ctx: crate::context::AppContext,
}

impl UserRepo {
    pub fn new(ctx: crate::context::AppContext) -> Self {
        Self { ctx }
    }

    /// Create a new user record with a token or overwrite an existing user's token.
    pub fn upsert_user_token(&self, user_id: &str, token: &str) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare("INSERT OR REPLACE INTO users (user_id, token) VALUES (?, ?)")?
            .execute(&[user_id, token])?;

        Ok(())
    }

    /// Try to find a user's auth token.
    pub fn get_token_by_user_id(&self, user_id: &str) -> crate::Result<String> {
        self.ctx
            .db
            .get()?
            .prepare("SELECT token FROM users WHERE user_id = ? LIMIT 1")?
            .query_row(&[user_id], |row| Ok(row.get(0)?))
            .map_err(|err| err.into())
    }
}
