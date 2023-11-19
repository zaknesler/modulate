use crate::{context::AppContext, util};
use futures::TryStreamExt;
use rspotify::{
    clients::OAuthClient,
    model::{PlayableId, PlaylistId},
    Token,
};
use std::sync::Arc;

pub async fn init(ctx: Arc<AppContext>) -> crate::Result<()> {
    loop {
        let now = tokio::time::Instant::now();
        let execute_in = std::time::Duration::from_secs(60 * 30);

        tokio::time::sleep_until(now + execute_in).await;

        if let Err(err) = execute(ctx.clone()).await {
            return Err(err);
        }
    }
}

async fn execute(ctx: Arc<AppContext>) -> crate::Result<()> {
    let watchers = ctx
        .db
        .get()?
        .prepare("SELECT watchers.user_id, watchers.playlist_id, users.token FROM watchers LEFT JOIN users ON watchers.user_id = users.user_id")?
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .collect::<rusqlite::Result<Vec<(String, String, String)>>>()?;

    for (user_id, playlist_id, token) in watchers {
        let token = serde_json::from_str::<Token>(&token)?;
        let client = util::client::get_token_ensure_refreshed(user_id, &token, ctx.clone()).await?;

        // Get all
        let saved_track_ids = client
            .current_user_saved_tracks(None)
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .filter_map(|track| track.track.id)
            .collect::<Vec<_>>();

        // TODO: get all items in playlist and remove those in saved_track_ids that already exist (to prevent duplicates)

        // Don't do anything if there are no saved tracks
        if saved_track_ids.len() == 0 {
            continue;
        }

        // Add tracks to playlist
        client
            .playlist_add_items(
                PlaylistId::from_id_or_uri(&playlist_id)?,
                saved_track_ids
                    .iter()
                    .map(|id| PlayableId::Track(id.clone())),
                None,
            )
            .await?;

        // Remove tracks from saved tracks
        client
            .current_user_saved_tracks_delete(saved_track_ids)
            .await?;
    }

    Ok(())
}
