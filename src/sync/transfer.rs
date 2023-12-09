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

        match (&watcher.playlist_from, &watcher.playlist_to) {
            (PlaylistType::Saved, PlaylistType::Id(playlist_to)) => {
                // Get all saved tracks and only continue if we have tracks to transfer
                let saved_track_ids = self.get_saved_track_ids().await?;
                if saved_track_ids.is_empty() {
                    return Ok(false);
                }

                // Get the tracks already in the target playlist to prevent duplicates
                let playlist_track_ids = self.get_playlist_track_ids(playlist_to).await?;

                // Get only the saved tracks that are not already in the target playlist and add them
                let ids_to_insert = self.get_ids_to_insert(&saved_track_ids, &playlist_track_ids);
                if !ids_to_insert.is_empty() {
                    self.client.playlist_add_ids(playlist_to, ids_to_insert.as_slice()).await?;
                }

                // Remove all saved tracks
                if watcher.should_remove {
                    self.client
                        .current_user_saved_tracks_remove_ids(
                            saved_track_ids
                                .iter()
                                .map(|id| id.as_ref())
                                .collect::<Vec<_>>()
                                .as_slice(),
                        )
                        .await?;
                }
            }
            (PlaylistType::Id(playlist_from), PlaylistType::Id(playlist_to)) => {
                // Verify that we aren't trying to transfer to the same playlist
                if playlist_from == playlist_to {
                    return Err(SyncError::InvalidTransferError(
                        "cannot transfer to the same playlist".to_owned(),
                    ));
                }

                // Get all tracks in the source playlist and only continue if we have tracks to transfer
                let from_track_ids = self.get_playlist_track_ids(playlist_from).await?;
                if from_track_ids.is_empty() {
                    return Ok(false);
                }

                // Get the tracks already in the target playlist to prevent duplicates
                let to_track_ids = self.get_playlist_track_ids(playlist_to).await?;

                // Get only the tracks that are not already in the target playlist and add them
                let ids_to_insert = self.get_ids_to_insert(&from_track_ids, &to_track_ids);
                if !ids_to_insert.is_empty() {
                    self.client.playlist_add_ids(playlist_to, ids_to_insert.as_slice()).await?;
                }

                // Remove all tracks from original playlist
                if watcher.should_remove {
                    self.client
                        .playlist_remove_ids(
                            playlist_from,
                            from_track_ids
                                .iter()
                                .map(|id| id.as_ref())
                                .collect::<Vec<_>>()
                                .as_slice(),
                        )
                        .await?;
                }
            }
            _ => return Err(SyncError::UnsupportedTransferError),
        }

        Ok(true)
    }

    /// Fetch the unique IDs in the user's saved tracks
    async fn get_saved_track_ids(&self) -> SyncResult<HashSet<String>> {
        Ok(self
            .client
            .current_user_saved_track_partials()
            .await?
            .into_iter()
            .map(|track| track.id)
            .collect::<HashSet<_>>())
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
