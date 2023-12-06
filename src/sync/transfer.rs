use crate::{
    api::client::Client,
    context::AppContext,
    model::{playlist::PlaylistType, watcher::Watcher},
    CONFIG,
};
use anyhow::anyhow;
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
    pub async fn try_transfer(&self, watcher: &Watcher) -> crate::Result<bool> {
        if !CONFIG.sync.enabled {
            return Ok(false);
        }

        match (&watcher.playlist_from, &watcher.playlist_to) {
            (PlaylistType::Saved, PlaylistType::Uri(to_uri)) => {
                // Get all saved tracks
                let saved_track_ids = self.get_saved_track_ids().await?;

                // Don't do anything if there are no saved tracks
                if saved_track_ids.is_empty() {
                    return Ok(false);
                }

                // Get IDs from current playlist and remove any from the saved tracks to prevent duplicates
                let playlist_track_ids = self.client.get_playlist_track_ids(&to_uri).await?;

                // Get only the saved tracks that are not already in the playlist
                let mut ids_to_insert =
                    saved_track_ids.difference(&playlist_track_ids).collect::<Vec<_>>();

                // Since we read them in order from newest to oldest, we want to insert them oldest first so we retain this order
                ids_to_insert.reverse();

                // Add all new tracks to playlist
                if !ids_to_insert.is_empty() {
                    self.client.playlist_add_items(to_id, ids_to_insert, None).await?;
                }

                // Remove all saved tracks
                if watcher.should_remove {
                    self.client.current_user_saved_tracks_delete(saved_track_ids).await?;
                }
            }
            (PlaylistType::Uri(from_uri), PlaylistType::Uri(to_uri)) => {
                if from_uri == to_uri {
                    return Err(crate::error::Error::InvalidTransfer(
                        "cannot transfer to the same playlist".to_owned(),
                    ));
                }

                let from_track_ids = self.client.get_playlist_track_ids(&from_uri).await?;

                // Don't do anything if there are no tracks in the playlist
                if from_track_ids.is_empty() {
                    return Ok(false);
                }

                let to_track_ids = self.client.get_playlist_track_ids(&to_uri).await?;

                // Get only the saved tracks that are not already in the playlist
                let mut ids_to_insert =
                    from_track_ids.difference(&to_track_ids).collect::<Vec<_>>();

                // Since we read them in order from newest to oldest, we want to insert them oldest first so we retain this order
                ids_to_insert.reverse();

                // Add all new tracks to playlist
                if !ids_to_insert.is_empty() {
                    self.client.playlist_add_items(to_uri, ids_to_insert, None).await?;
                }

                // Remove all tracks from original playlist
                if watcher.should_remove {
                    self.client
                        .playlist_remove_all_occurrences_of_items(
                            from_uri.clone(),
                            from_track_ids,
                            None,
                        )
                        .await
                        .map_err(|err| match &err {
                            // rspotify::ClientError::Http(inner_err) => match inner_err.as_ref() {
                            //     rspotify::http::HttpError::StatusCode(res)
                            //         if res.status().as_u16() == 403 =>
                            //     {
                            //         crate::error::Error::CouldNotRemoveTracks(from_id.to_string())
                            //     }
                            //     _ => err.into(),
                            // },
                            _ => err.into(),
                        })?;
                }
            }
            _ => return Err(anyhow!("unsupported transfer type").into()),
        }

        Ok(true)
    }

    async fn get_saved_track_ids(&self) -> crate::Result<HashSet<String>> {
        Ok(self
            .client
            .get_current_user_saved_tracks()
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .filter_map(|track| track.track.id)
            .collect::<HashSet<_>>())
    }
}
