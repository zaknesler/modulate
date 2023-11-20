use crate::context::AppContext;
use anyhow::anyhow;
use futures::TryStreamExt;
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{PlayableId, PlaylistId, TrackId},
    AuthCodeSpotify,
};
use std::collections::HashSet;

pub enum PlaylistType<'a> {
    Saved,
    WithId(PlaylistId<'a>),
}

#[allow(dead_code)]
pub struct PlaylistTransfer {
    ctx: AppContext,
    client: AuthCodeSpotify,
}

impl PlaylistTransfer {
    pub fn new(ctx: AppContext, client: AuthCodeSpotify) -> Self {
        Self { ctx, client }
    }

    /// Transfer tracks from one playlist to another, regardless of playlist type
    pub async fn transfer(
        &self,
        from: PlaylistType<'_>,
        to: PlaylistType<'_>,
    ) -> crate::Result<bool> {
        Ok(match (from, to) {
            (PlaylistType::Saved, PlaylistType::WithId(playlist_id)) => {
                // Get all saved tracks
                let saved_track_ids = self.get_saved_track_ids().await?;

                // Don't do anything if there are no saved tracks
                if saved_track_ids.is_empty() {
                    return Ok(false);
                }

                // Get IDs from current playlist and remove any from the saved tracks to prevent duplicates
                let playlist_track_ids = self.get_playlist_track_ids(playlist_id.clone()).await?;

                // Get only the saved tracks that are not already in the playlist
                let ids_to_insert = saved_track_ids
                    .difference(&playlist_track_ids)
                    .map(|id| PlayableId::Track(id.clone()))
                    .collect::<Vec<_>>();

                // Add all new tracks to playlist
                if !ids_to_insert.is_empty() {
                    self.client
                        .playlist_add_items(playlist_id, ids_to_insert, None)
                        .await?;
                }

                // Remove all saved tracks
                self.client
                    .current_user_saved_tracks_delete(saved_track_ids)
                    .await?;

                true
            }
            (PlaylistType::WithId(from_id), PlaylistType::WithId(to_id)) => {
                let from_track_ids = self.get_playlist_track_ids(from_id.clone()).await?;

                // Don't do anything if there are no tracks in the playlist
                if from_track_ids.is_empty() {
                    return Ok(false);
                }

                let to_track_ids = self.get_playlist_track_ids(to_id.clone()).await?;

                // Get only the saved tracks that are not already in the playlist
                let ids_to_insert = from_track_ids
                    .difference(&to_track_ids)
                    .map(|id| PlayableId::Track(id.clone()))
                    .collect::<Vec<_>>();

                // Add all new tracks to playlist
                if !ids_to_insert.is_empty() {
                    self.client
                        .playlist_add_items(to_id, ids_to_insert, None)
                        .await?;
                }

                // Remove all tracks from original playlist
                self.client
                    .playlist_remove_all_occurrences_of_items(
                        from_id,
                        from_track_ids
                            .iter()
                            .map(|id| PlayableId::Track(id.clone())),
                        None,
                    )
                    .await?;

                true
            }
            _ => return Err(anyhow!("unsupported transfer type").into()),
        })
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
