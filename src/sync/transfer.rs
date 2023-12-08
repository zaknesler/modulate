use super::error::{SyncError, SyncResult};
use crate::{
    api::client::Client,
    context::AppContext,
    db::model::{playlist::PlaylistType, watcher::Watcher},
    CONFIG,
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
        if !CONFIG.sync.enabled {
            return Ok(false);
        }

        match (&watcher.playlist_from, &watcher.playlist_to) {
            (PlaylistType::Saved, PlaylistType::Id(playlist_to)) => {
                // Get all saved tracks
                let saved_track_ids = self.get_saved_track_ids().await?;

                // Don't do anything if there are no saved tracks
                if saved_track_ids.is_empty() {
                    return Ok(false);
                }

                // Get IDs from current playlist and remove any from the saved tracks to prevent duplicates
                let playlist_track_ids = self
                    .client
                    .playlist_track_partials(playlist_to)
                    .await?
                    .into_iter()
                    .map(|track| track.id)
                    .collect::<HashSet<_>>();

                // Get only the saved tracks that are not already in the playlist
                let mut ids_to_insert = saved_track_ids
                    .difference(&playlist_track_ids)
                    .map(|val| val.as_ref())
                    .collect::<Vec<_>>();

                // Since we read them in order from newest to oldest, we want to insert them oldest first so we retain this order
                ids_to_insert.reverse();

                // Add all new tracks to playlist
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
                if playlist_from == playlist_to {
                    return Err(SyncError::InvalidTransferError(
                        "cannot transfer to the same playlist".to_owned(),
                    ));
                }

                let from_track_ids = self
                    .client
                    .playlist_track_partials(&playlist_from)
                    .await?
                    .into_iter()
                    .map(|track| track.id)
                    .collect::<HashSet<_>>();

                // Don't do anything if there are no tracks in the playlist
                if from_track_ids.is_empty() {
                    return Ok(false);
                }

                let to_track_ids = self
                    .client
                    .playlist_track_partials(&playlist_to)
                    .await?
                    .into_iter()
                    .map(|track| track.id)
                    .collect::<HashSet<_>>();

                // Get only the saved tracks that are not already in the playlist
                let mut ids_to_insert = from_track_ids
                    .difference(&to_track_ids)
                    .map(|val| val.as_ref())
                    .collect::<Vec<_>>();

                // Since we read them in order from newest to oldest, we want to insert them oldest first so we retain this order
                ids_to_insert.reverse();

                // Add all new tracks to playlist
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

    async fn get_saved_track_ids(&self) -> SyncResult<HashSet<String>> {
        Ok(self
            .client
            .current_user_saved_track_partials()
            .await?
            .into_iter()
            .map(|track| track.id)
            .collect::<HashSet<_>>())
    }
}
