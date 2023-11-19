use crate::{context::AppContext, util};
use futures::TryStreamExt;
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{PlayableId, PlaylistId},
    Token,
};
use std::collections::HashSet;

/// For the user with the given ID, transfer their saved tracks to the playlist with the given ID.
pub async fn sync_user_playlist(
    user_id: &str,
    playlist_id: &str,
    token: &Token,
    ctx: AppContext,
) -> crate::Result<bool> {
    let client =
        util::client::get_token_ensure_refreshed(user_id.to_owned(), token, ctx.clone()).await?;

    let playlist_id = PlaylistId::from_id_or_uri(playlist_id)?;

    // Get all
    let saved_track_ids = client
        .current_user_saved_tracks(None)
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .filter_map(|track| track.track.id)
        .collect::<HashSet<_>>();

    // Don't do anything if there are no saved tracks
    if saved_track_ids.is_empty() {
        return Ok(false);
    }

    // Get IDs from current playlist and remove any from the saved tracks to prevent duplicates
    let playlist_track_ids = client
        .playlist_items(playlist_id.clone(), None, None)
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
        .collect::<HashSet<_>>();

    // Get only the saved tracks that are not already in the playlist
    let ids_to_insert = saved_track_ids
        .difference(&playlist_track_ids)
        .map(|id| PlayableId::Track(id.clone()))
        .collect::<Vec<_>>();

    // Add all new tracks to playlist
    if !ids_to_insert.is_empty() {
        client
            .playlist_add_items(playlist_id, ids_to_insert, None)
            .await?;
    }

    // Remove all saved tracks
    client
        .current_user_saved_tracks_delete(saved_track_ids)
        .await?;

    Ok(true)
}
