pub struct WatcherRepo {
    ctx: crate::context::AppContext,
}

impl WatcherRepo {
    pub fn new(ctx: crate::context::AppContext) -> Self {
        Self { ctx }
    }

    /// Try to find the ID of the playlist a user has configured to watch.
    pub fn get_watched_playlist_id_by_user_id(
        &self,
        user_id: &str,
    ) -> crate::Result<Option<String>> {
        Ok(self
            .ctx
            .db
            .get()?
            .prepare("SELECT playlist_id FROM watchers WHERE user_id = ? LIMIT 1")?
            .query_row(&[user_id], |row| Ok(row.get(0)?))
            .ok())
    }

    /// Get all configured watchers.
    pub fn get_all_watchers(&self) -> crate::Result<Vec<(String, String, String)>> {
        self.ctx
            .db
            .get() ?
            .prepare("SELECT watchers.user_id, watchers.playlist_id, users.token FROM watchers LEFT JOIN users ON watchers.user_id = users.user_id")?
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|err| err.into())
    }

    /// Create a watcher for a user and playlist.
    pub fn create_watcher(&self, user_id: &str, playlist_id: &str) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare("INSERT INTO watchers (user_id, playlist_id) VALUES (?, ?)")?
            .execute(&[user_id, playlist_id])?;

        Ok(())
    }

    /// Delete a user's watcher.
    pub fn delete_watcher(&self, user_id: &str) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare("DELETE FROM watchers WHERE user_id = ?")?
            .execute(&[user_id])?;

        Ok(())
    }
}
