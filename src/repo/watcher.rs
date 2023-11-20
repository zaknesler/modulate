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
            .prepare(
                "SELECT users.id, users.token, watchers.from_playlist, watchers.to_playlist, watchers.should_remove FROM watchers
                LEFT JOIN users ON users.id = watchers.user_id",
            )?
            .query_map([], |row| {
                Ok(Watcher::try_from_row_data(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get::<usize, u8>(4)? != 0,
                ).unwrap())
            })?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|err| err.into())
    }

    /// Get all watchers for a given user ID.
    pub fn get_all_watchers_by_user(&self, user_id: &str) -> crate::Result<Vec<Watcher>> {
        self.ctx
            .db
            .get()?
            .prepare(
                "SELECT users.token, watchers.from_playlist, watchers.to_playlist, watchers.should_remove FROM watchers
                LEFT JOIN users ON users.id = watchers.user_id
                AND watchers.user_id = ?",
            )?
            .query_map(&[user_id], |row| {
                Ok(Watcher::try_from_row_data(
                    user_id.to_owned(),
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get::<usize, u8>(3)? != 0,
                ).unwrap())
            })?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|err| err.into())
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
                "INSERT INTO watchers
                (user_id, from_playlist, to_playlist, should_remove, created_at)
                VALUES (?, ?, ?, 1, datetime())",
            )?
            .execute(&[user_id, &from.to_string(), &to.to_string()])?;

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
                "DELETE FROM watchers WHERE user_id = ? AND from_playlist = ? to_playlist = ?",
            )?
            .execute(&[user_id, &from.to_string(), &to.to_string()])?;

        Ok(())
    }
}
