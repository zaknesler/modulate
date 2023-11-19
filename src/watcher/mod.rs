use std::collections::HashSet;

use crate::{context::AppContext, repo::watcher::WatcherRepo, util, CONFIG};
use futures::TryStreamExt;
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{PlayableId, PlaylistId},
    Token,
};

pub async fn init(ctx: AppContext) -> crate::Result<()> {
    loop {
        let now = tokio::time::Instant::now();
        let execute_in = std::time::Duration::from_secs(60 * CONFIG.sync.interval_mins as u64);

        tokio::time::sleep_until(now + execute_in).await;

        if let Err(err) = execute(ctx.clone()).await {
            return Err(err);
        }
    }
}

async fn execute(ctx: AppContext) -> crate::Result<()> {
    let watcher_repo = WatcherRepo::new(ctx.clone());

    for (user_id, playlist_id, token) in watcher_repo.get_all_watchers()? {
        let token = serde_json::from_str::<Token>(&token)?;
        let client = util::client::get_token_ensure_refreshed(user_id, &token, ctx.clone()).await?;

        let playlist_id = PlaylistId::from_id_or_uri(&playlist_id)?;

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
            continue;
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
    }

    Ok(())
}
