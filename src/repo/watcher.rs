use rusqlite::params;

use crate::model::{playlist::PlaylistType, watcher::Watcher};

pub struct WatcherRepo {
    ctx: crate::context::AppContext,
}

impl WatcherRepo {
    pub fn new(ctx: crate::context::AppContext) -> Self {
        Self { ctx }
    }

    /// Get all configured watchers.
    pub fn get_all_watchers(&self) -> crate::Result<Vec<Watcher>> {
        self.ctx
            .db
            .get()?
            .prepare("
                SELECT watchers.id, users.user_id, users.token, watchers.playlist_from, watchers.playlist_to, watchers.should_remove
                FROM watchers
                INNER JOIN users
                ON users.user_id = watchers.user_id
            ")?
            .query_map([], |row| Ok(Watcher::try_from_row_data(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?).unwrap()))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|err| err.into())
    }

    /// Get all watchers for a given user ID.
    pub fn get_all_watchers_by_user(&self, user_id: &str) -> crate::Result<Vec<Watcher>> {
        self.ctx
            .db
            .get()?
            .prepare("
                SELECT watchers.id, watchers.user_id, users.token, watchers.playlist_from, watchers.playlist_to, watchers.should_remove
                FROM watchers
                INNER JOIN users
                ON users.user_id = watchers.user_id
                WHERE watchers.user_id = ?
            ")?
            .query_map(params![user_id], |row| Ok(Watcher::try_from_row_data(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?).unwrap()))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|err| err.into())
    }

    /// Get specific watcher for a given ID and user ID.
    pub fn get_watcher_by_id_and_user(&self, id: i64, user_id: &str) -> crate::Result<Watcher> {
        self.ctx
            .db
            .get()?
            .prepare("
                SELECT watchers.id, watchers.user_id, users.token, watchers.playlist_from, watchers.playlist_to, watchers.should_remove
                FROM watchers
                INNER JOIN users
                ON users.user_id = watchers.user_id
                WHERE watchers.id = ? AND watchers.user_id = ?
            ")?
            .query_row(params![id, user_id], |row| Ok(Watcher::try_from_row_data(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?).unwrap()))
            .map_err(|err|err.into())
    }

    /// Create a watcher for a user and playlist.
    pub fn create_watcher(
        &self,
        user_id: &str,
        from: PlaylistType,
        to: PlaylistType,
    ) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare(
                "INSERT INTO watchers (user_id, playlist_from, playlist_to, should_remove, created_at) VALUES (?, ?, ?, 1, datetime())",
            )?
            .execute(params![user_id, from.to_value(), to.to_value()])?;

        Ok(())
    }

    /// Delete a watcher given user and playlist IDs.
    pub fn delete_watcher(
        &self,
        user_id: &str,
        from: PlaylistType,
        to: PlaylistType,
    ) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare(
                "DELETE FROM watchers WHERE user_id = ? AND playlist_from = ? AND playlist_to = ?",
            )?
            .execute(params![user_id, from.to_value(), to.to_value()])?;

        Ok(())
    }
}
