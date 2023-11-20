use crate::{context::AppContext, repo::watcher::WatcherRepo, util::client, CONFIG};
use rspotify::model::PlaylistId;

pub mod transfer;

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
    let watchers = watcher_repo.get_all_watchers()?;

    tracing::info!("Syncing playlists of {} user(s)...", watchers.len());

    for (user_id, playlist_id, token) in watchers {
        let token = serde_json::from_str::<rspotify::Token>(&token)?;
        let client =
            client::get_token_ensure_refreshed(user_id.to_owned(), &token, ctx.clone()).await?;

        // Transfer tracks from saved playlist to playlist with the given ID
        transfer::PlaylistTransfer::new(ctx.clone(), client)
            .transfer(
                transfer::PlaylistType::Saved,
                transfer::PlaylistType::WithId(PlaylistId::from_id_or_uri(&playlist_id)?),
            )
            .await?;
    }

    tracing::info!("Synced");

    Ok(())
}
