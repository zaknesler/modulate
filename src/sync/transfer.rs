use super::error::{SyncError, SyncResult};
use crate::{
    api::{client::Client, id::PlaylistId},
    context::AppContext,
    db::model::{playlist::PlaylistType, watcher::Watcher},
};
use std::collections::HashSet;

#[allow(dead_code)]
pub struct PlaylistTransfer {
    ctx: AppContext,
    client: Client,
}

impl PlaylistTransfer {
    pub fn new(ctx: AppContext, client: Client) -> Self {
        Self { ctx, client }
    }

    /// Using data from a watcher, attempt to transfer tracks from one playlist to another.
    pub async fn try_transfer(&self, watcher: &Watcher) -> SyncResult<bool> {
        if !self.ctx.config.sync.enabled {
            return Ok(false);
        }

        if watcher.playlist_from == watcher.playlist_to {
            return Err(SyncError::InvalidTransferError(
                "cannot transfer to the same playlist".to_owned(),
            ));
        }

        // Get all tracks in source playlist and only continue if we have tracks to transfer
        let ids_to_transfer = self.get_track_ids_to_transfer(&watcher.playlist_from).await?;
        if ids_to_transfer.is_empty() {
            return Ok(false);
        }

        // Transfer all tracks not already in target playlist
        self.maybe_transfer_tracks(&watcher.playlist_to, &ids_to_transfer).await?;

        // Remove all original tracks from source playlist
        if watcher.should_remove {
            self.remove_tracks_from_playlist(&watcher.playlist_from, &ids_to_transfer)
                .await?;
        }

        Ok(true)
    }

    /// Get the tracks IDs in the source playlist
    async fn get_track_ids_to_transfer(
        &self,
        playlist_from: &PlaylistType,
    ) -> SyncResult<HashSet<String>> {
        let ids = match playlist_from {
            PlaylistType::Saved => self.client.current_user_saved_track_partials().await?,
            PlaylistType::Id(id) => self.client.playlist_track_partials(id).await?,
        };

        Ok(ids.into_iter().map(|track| track.id).collect::<HashSet<_>>())
    }

    /// Remove the tracks from the specified playlist by ID
    async fn remove_tracks_from_playlist(
        &self,
        playlist_from: &PlaylistType,
        ids_to_remove: &HashSet<String>,
    ) -> SyncResult<()> {
        let ids_to_remove = ids_to_remove.iter().map(|id| id.as_ref()).collect::<Vec<_>>();

        match playlist_from {
            PlaylistType::Saved => {
                self.client
                    .current_user_saved_tracks_remove_ids(ids_to_remove.as_slice())
                    .await?;
            }
            PlaylistType::Id(id) => {
                self.client.playlist_remove_ids(id, ids_to_remove.as_slice()).await?;
            }
        };

        Ok(())
    }

    /// Transfer the tracks to the target playlist by ID
    async fn maybe_transfer_tracks(
        &self,
        playlist_to: &PlaylistType,
        ids_to_transfer: &HashSet<String>,
    ) -> SyncResult<()> {
        match playlist_to {
            PlaylistType::Id(to_id) => {
                // Get the tracks already in the target playlist to prevent duplicates
                let playlist_track_ids = self.get_playlist_track_ids(to_id).await?;

                // Get only the saved tracks that are not already in the target playlist and add them
                let ids_to_insert = self.get_ids_to_insert(&ids_to_transfer, &playlist_track_ids);
                if !ids_to_insert.is_empty() {
                    self.client.playlist_add_ids(to_id, ids_to_insert.as_slice()).await?;
                }
            }
            PlaylistType::Saved => {
                // We don't want to support transferring to saved tracks (for now; I just don't see the point)
                return Err(SyncError::InvalidTransferError(
                    "cannot transfer to saved tracks".to_owned(),
                ));
            }
        };

        Ok(())
    }

    /// Fetch the unique IDs in the specified playlist
    async fn get_playlist_track_ids(&self, playlist: &PlaylistId) -> SyncResult<HashSet<String>> {
        Ok(self
            .client
            .playlist_track_partials(playlist)
            .await?
            .into_iter()
            .map(|track| track.id)
            .collect::<HashSet<_>>())
    }

    /// Find the IDs that are not in the target playlist, and return them reversed so they may be inserted in the correct order
    fn get_ids_to_insert<'a>(
        &self,
        from: &'a HashSet<String>,
        to: &'a HashSet<String>,
    ) -> Vec<&'a str> {
        let mut ids_to_insert =
            from.difference(&to).map(|track| track.as_ref()).collect::<Vec<_>>();

        // Since we read them in order from newest to oldest, we want to insert them oldest first so we retain this order
        ids_to_insert.reverse();

        ids_to_insert
    }
}
