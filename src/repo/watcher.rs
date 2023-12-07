use crate::model::{
    playlist::PlaylistType,
    watcher::{SyncInterval, Watcher, WATCHER_COLUMNS},
};
use chrono::Utc;
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
            .prepare(format!("SELECT {} FROM watchers", WATCHER_COLUMNS).as_ref())?
            .query_and_then([], |row| row.try_into())?
            .collect::<crate::Result<Vec<_>>>()
    }

    /// Get all watchers for a specific playlist.
    pub fn get_watchers_for_playlist(&self, from: &PlaylistType) -> crate::Result<Vec<Watcher>> {
        self.ctx
            .db
            .get()?
            .prepare(
                format!(
                    "SELECT {} FROM watchers WHERE watchers.playlist_from = ?1",
                    WATCHER_COLUMNS
                )
                .as_ref(),
            )?
            .query_and_then(params![from.to_value()], |row| row.try_into())?
            .collect::<crate::Result<Vec<_>>>()
    }

    /// Get all watchers for a given user ID.
    pub fn get_watchers_by_user(&self, user_uri: &str) -> crate::Result<Vec<Watcher>> {
        self.ctx
            .db
            .get()?
            .prepare(
                format!(
                    "SELECT {} FROM watchers WHERE watchers.user_uri = ?1",
                    WATCHER_COLUMNS
                )
                .as_ref(),
            )?
            .query_and_then(params![user_uri], |row| row.try_into())?
            .collect::<crate::Result<Vec<_>>>()
    }

    /// Get specific watcher for a given ID and user ID.
    pub fn get_watcher_by_id_and_user(
        &self,
        id: i64,
        user_uri: &str,
    ) -> crate::Result<Option<Watcher>> {
        Ok(self
            .ctx
            .db
            .get()?
            .prepare(
                format!(
                    "SELECT {} FROM watchers WHERE watchers.id = ?1 AND watchers.user_uri = ?2",
                    WATCHER_COLUMNS
                )
                .as_ref(),
            )?
            .query_and_then(params![id, user_uri], |row| Watcher::try_from(row))?
            .collect::<crate::Result<Vec<_>>>()?
            .first()
            .cloned())
    }

    /// Update the next_sync_at date of a watcher by ID.
    pub fn update_watcher_next_sync_at(
        &self,
        id: i64,
        next_sync_at: chrono::DateTime<chrono::Utc>,
    ) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare("UPDATE watchers SET next_sync_at = ?1 WHERE watchers.id = ?2")?
            .execute(params![next_sync_at.to_rfc3339(), id])?;

        Ok(())
    }

    /// Create a watcher for a user and playlist.
    pub fn create_watcher(
        &self,
        user_uri: &str,
        from: &PlaylistType,
        to: &PlaylistType,
        should_remove: bool,
        sync_interval: SyncInterval,
    ) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare("INSERT INTO watchers (user_uri, playlist_from, playlist_to, should_remove, sync_interval, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?
            .execute(params![user_uri, from.to_value(), to.to_value(), should_remove, sync_interval.to_string(), Utc::now().to_rfc3339()])?;

        Ok(())
    }

    /// Delete a watcher given user and playlist IDs.
    pub fn delete_watcher_by_user_and_playlists(
        &self,
        user_uri: &str,
        from: &PlaylistType,
        to: &PlaylistType,
    ) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare("DELETE FROM watchers WHERE user_uri = ?1 AND playlist_from = ?2 AND playlist_to = ?3")?
            .execute(params![user_uri, from.to_value(), to.to_value()])?;

        Ok(())
    }

    /// Delete all watchers given a user_uri.
    pub fn delete_all_watchers_by_user(&self, user_uri: &str) -> crate::Result<()> {
        self.ctx
            .db
            .get()?
            .prepare("DELETE FROM watchers WHERE user_uri = ?1")?
            .execute(params![user_uri])?;

        Ok(())
    }
}
