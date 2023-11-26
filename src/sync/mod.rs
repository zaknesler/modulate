use crate::{
    context::AppContext,
    repo::{user::UserRepo, watcher::WatcherRepo},
    util::client,
    CONFIG,
};

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
    let user_repo = UserRepo::new(ctx.clone());
    let watcher_repo = WatcherRepo::new(ctx.clone());
    let watchers = watcher_repo.get_all_watchers()?;

    tracing::info!("Syncing playlists of {} user(s)...", watchers.len());

    for watcher in watchers {
        let user_token: rspotify::Token =
            serde_json::from_str(&user_repo.get_token_by_user_id(&watcher.user_id)?)?;
        let (client, _) =
            client::get_token_ensure_refreshed(watcher.user_id.clone(), &user_token, ctx.clone())
                .await?;

        transfer::PlaylistTransfer::new(ctx.clone(), client)
            .try_transfer(&watcher)
            .await?;
    }

    tracing::info!("Synced");

    Ok(())
}
