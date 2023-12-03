use crate::{
    context::AppContext,
    model::{playlist::PlaylistType, watcher::Watcher},
    CONFIG,
};
use anyhow::anyhow;
use futures::TryStreamExt;
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{PlayableId, PlaylistId, TrackId},
    AuthCodeSpotify,
};
use std::collections::HashSet;

#[allow(dead_code)]
pub struct PlaylistTransfer {
    ctx: AppContext,
    client: AuthCodeSpotify,
}

impl PlaylistTransfer {
    pub fn new(ctx: AppContext, client: AuthCodeSpotify) -> Self {
        Self { ctx, client }
    }

    /// Using data from a watcher, attempt to transfer tracks from one playlist to another.
    pub async fn try_transfer(&self, watcher: &Watcher) -> crate::Result<bool> {
        if !CONFIG.sync.enabled {
            return Ok(false);
        }

        match (&watcher.playlist_from, &watcher.playlist_to) {
            (PlaylistType::Saved, PlaylistType::Uri(to_id)) => {
                let to_id = PlaylistId::from_id_or_uri(&to_id)?;

                // Get all saved tracks
                let saved_track_ids = self.get_saved_track_ids().await?;

                // Don't do anything if there are no saved tracks
                if saved_track_ids.is_empty() {
                    return Ok(false);
                }

                // Get IDs from current playlist and remove any from the saved tracks to prevent duplicates
                let playlist_track_ids = self.get_playlist_track_ids(to_id.clone()).await?;

                // Get only the saved tracks that are not already in the playlist
                let mut ids_to_insert = saved_track_ids
                    .difference(&playlist_track_ids)
                    .map(|id| PlayableId::Track(id.clone()))
                    .collect::<Vec<_>>();

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
            (PlaylistType::Uri(from_id), PlaylistType::Uri(to_id)) => {
                if from_id == to_id {
                    return Err(crate::error::Error::InvalidTransfer(
                        "cannot transfer to the same playlist".to_owned(),
                    ));
                }

                let from_id = PlaylistId::from_id_or_uri(&from_id)?;
                let to_id = PlaylistId::from_id_or_uri(&to_id)?;

                let from_track_ids = self.get_playlist_track_ids(from_id.clone()).await?;

                // Don't do anything if there are no tracks in the playlist
                if from_track_ids.is_empty() {
                    return Ok(false);
                }

                let to_track_ids = self.get_playlist_track_ids(to_id.clone()).await?;

                // Get only the saved tracks that are not already in the playlist
                let mut ids_to_insert = from_track_ids
                    .difference(&to_track_ids)
                    .map(|id| PlayableId::Track(id.clone()))
                    .collect::<Vec<_>>();

                // Since we read them in order from newest to oldest, we want to insert them oldest first so we retain this order
                ids_to_insert.reverse();

                // Add all new tracks to playlist
                if !ids_to_insert.is_empty() {
                    self.client.playlist_add_items(to_id, ids_to_insert, None).await?;
                }

                // Remove all tracks from original playlist
                if watcher.should_remove {
                    self.client
                        .playlist_remove_all_occurrences_of_items(
                            from_id.clone(),
                            from_track_ids.iter().map(|id| PlayableId::Track(id.clone())),
                            None,
                        )
                        .await
                        .map_err(|err| match &err {
                            rspotify::ClientError::Http(inner_err) => match inner_err.as_ref() {
                                rspotify::http::HttpError::StatusCode(res)
                                    if res.status().as_u16() == 403 =>
                                {
                                    crate::error::Error::CouldNotRemoveTracks(from_id.to_string())
                                }
                                _ => err.into(),
                            },
                            _ => err.into(),
                        })?;
                }
            }
            _ => return Err(anyhow!("unsupported transfer type").into()),
        }

        Ok(true)
    }

    async fn get_saved_track_ids(&self) -> crate::Result<HashSet<TrackId<'_>>> {
        Ok(self
            .client
            .current_user_saved_tracks(None)
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .filter_map(|track| track.track.id)
            .collect::<HashSet<_>>())
    }

    async fn get_playlist_track_ids(
        &self,
        id: PlaylistId<'_>,
    ) -> crate::Result<HashSet<TrackId<'_>>> {
        Ok(self
            .client
            .playlist_items(id, None, None)
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|item| item.track)
            .filter_map(|track| match track {
                rspotify::model::PlayableItem::Track(track) => track.id,
                _ => None,
            })
            .collect::<HashSet<_>>())
    }
}
