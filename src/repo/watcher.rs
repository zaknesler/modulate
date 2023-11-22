use crate::model::{playlist::PlaylistType, watcher::Watcher};
use rusqlite::params;

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
                SELECT watchers.id, watchers.user_id, watchers.playlist_from, watchers.playlist_to, watchers.should_remove
                FROM watchers
            ")?
            .query_map([], |row| Ok(Watcher::try_from_row_data(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?).unwrap()))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|err| err.into())
    }

    /// Get all watchers for a given user ID.
    pub fn get_all_watchers_by_user(&self, user_id: &str) -> crate::Result<Vec<Watcher>> {
        self.ctx
            .db
            .get()?
            .prepare("
                SELECT watchers.id, watchers.user_id, watchers.playlist_from, watchers.playlist_to, watchers.should_remove
                FROM watchers
                WHERE watchers.user_id = ?1
            ")?
            .query_map(params![user_id], |row| Ok(Watcher::try_from_row_data(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, ).unwrap()))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|err| err.into())
    }

    /// Get specific watcher for a given ID and user ID.
    pub fn get_watcher_by_id_and_user(&self, id: i64, user_id: &str) -> crate::Result<Watcher> {
        self.ctx
            .db
            .get()?
            .prepare("
                SELECT watchers.id, watchers.user_id, watchers.playlist_from, watchers.playlist_to, watchers.should_remove
                FROM watchers
                WHERE watchers.id = ?1 AND watchers.user_id = ?2
            ")?
            .query_row(params![id, user_id], |row| Ok(Watcher::try_from_row_data(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?).unwrap()))
            .map_err(|err|err.into())
    }

    /// Create a watcher for a user and playlist.
    pub fn create_watcher(
        &self,
        user_id: &str,
        from: PlaylistType,
        to: PlaylistType,
        should_remove: bool,
    ) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare(
                "INSERT INTO watchers (user_id, playlist_from, playlist_to, should_remove, created_at)
                VALUES (?1, ?2, ?3, ?4, datetime())",
            )?
            .execute(params![user_id, from.to_value(), to.to_value(), should_remove])?;

        Ok(())
    }

    /// Delete a watcher given user and playlist IDs.
    pub fn delete_watcher_by_user_and_playlists(
        &self,
        user_id: &str,
        from: PlaylistType,
        to: PlaylistType,
    ) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare(
                "DELETE FROM watchers WHERE user_id = ?1 AND playlist_from = ?2 AND playlist_to = ?3",
            )?
            .execute(params![user_id, from.to_value(), to.to_value()])?;

        Ok(())
    }

    /// Delete all watchers given a user_id.
    pub fn delete_all_watchers_by_user(&self, user_id: &str) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare("DELETE FROM watchers WHERE user_id = ?1")?
            .execute(params![user_id])?;

        Ok(())
    }
}
