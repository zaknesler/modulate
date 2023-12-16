use crate::{
    db::{
        error::DbResult,
        model::transfer::{Transfer, COLUMNS},
    },
    sync::error::SyncError,
};
use chrono::{DateTime, Utc};
use rusqlite::params;

pub struct TransferRepo {
    ctx: crate::context::AppContext,
}

impl TransferRepo {
    pub fn new(ctx: crate::context::AppContext) -> Self {
        Self { ctx }
    }

    /// Create a transfer record with a list of errors
    pub fn log_transfer(
        &self,
        watcher_id: u32,
        num_tracks_transferred: &u32,
        error: &Option<&SyncError>,
        synced_at: DateTime<Utc>,
    ) -> DbResult<()> {
        self.ctx
            .db
            .get()?
            .prepare(
                "INSERT INTO transfers (watcher_id, num_tracks_transferred, error, synced_at, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            )?
            .execute(params![
                watcher_id,
                num_tracks_transferred,
                error.map(|err| err.to_string()).unwrap_or_default(),
                synced_at.to_rfc3339(),
                chrono::Utc::now().to_rfc3339()
            ])?;

        Ok(())
    }

    /// Fetch all transfers for a watcher by ID.
    #[allow(dead_code)]
    pub fn get_transfers_for_watcher(&self, id: u32) -> DbResult<Vec<Transfer>> {
        Ok(self
            .ctx
            .db
            .get()?
            .prepare(format!("SELECT {COLUMNS} FROM transfers WHERE transfers.id = ?1").as_ref())?
            .query_and_then(params![id], |row| Transfer::try_from(row))?
            .collect::<DbResult<Vec<_>>>()?)
    }
}
