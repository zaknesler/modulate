use self::error::SyncResult;
use crate::{
    api::client,
    context::AppContext,
    db::repo::{user::UserRepo, watcher::WatcherRepo},
};
use chrono::{Timelike, Utc};

pub mod error;
pub mod transfer;

/// Interval to fetch watchers to see if any need to be run again
const CHECK_INTERVAL_MINS: i64 = 1;

pub async fn init(ctx: AppContext) -> SyncResult<()> {
    loop {
        let now = Utc::now();
        let next_update = now
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            .checked_add_signed(chrono::Duration::minutes(CHECK_INTERVAL_MINS))
            .unwrap();

        let time_until_next_update = tokio::time::Duration::from_millis(
            (next_update - now).num_milliseconds().try_into().unwrap(),
        );

        tracing::info!(
            "{:.0}s until next check",
            time_until_next_update.as_secs_f64()
        );

        tokio::time::sleep_until(
            tokio::time::Instant::now().checked_add(time_until_next_update).unwrap(),
        )
        .await;

        if let Err(err) = execute(ctx.clone()).await {
            return Err(err);
        }
    }
}

async fn execute(ctx: AppContext) -> SyncResult<()> {
    let user_repo = UserRepo::new(ctx.clone());
    let watcher_repo = WatcherRepo::new(ctx.clone());
    let watchers = watcher_repo.get_all_watchers()?;

    let to_sync = watchers
        .into_iter()
        .filter(|watcher| {
            watcher.next_sync_at.is_none()
                || watcher.next_sync_at.is_some_and(|next_sync| next_sync <= Utc::now())
        })
        .collect::<Vec<_>>();

    if to_sync.is_empty() {
        return Ok(());
    }

    tracing::info!("Syncing {} watcher(s)...", to_sync.len());

    let now = Utc::now().with_second(0).unwrap().with_nanosecond(0).unwrap();

    for watcher in to_sync {
        let user_token = user_repo.get_token_by_user_uri(&watcher.user_uri)?;
        let client = client::Client::new_with_token(user_token)?;

        transfer::PlaylistTransfer::new(ctx.clone(), client)
            .try_transfer(&watcher)
            .await?;

        watcher_repo.update_watcher_next_sync_at(
            watcher.id,
            now.checked_add_signed(watcher.sync_interval.into()).unwrap(),
        )?;
    }

    tracing::info!("Synced");

    Ok(())
}
