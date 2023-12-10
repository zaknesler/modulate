use self::error::SyncResult;
use crate::{
    api::client,
    context::AppContext,
    db::{
        model::watcher::Watcher,
        repo::{user::UserRepo, watcher::WatcherRepo},
    },
    sync::error::SyncError,
};
use chrono::{DateTime, Timelike, Utc};

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

        // Kill thread if worker task errored
        execute(ctx.clone()).await?;
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
        if let Err(err) = sync_watcher(ctx.clone(), &user_repo, &watcher_repo, &watcher, now).await
        {
            // Don't kill worker thread if an individual sync task errored
            tracing::error!("Error when syncing watcher: {}", err);
            sentry::capture_error(&err);
        }
    }

    tracing::info!("Synced");

    Ok(())
}

async fn sync_watcher(
    ctx: AppContext,
    user_repo: &UserRepo,
    watcher_repo: &WatcherRepo,
    watcher: &Watcher,
    now: DateTime<Utc>,
) -> SyncResult<()> {
    let user = user_repo
        .find_user_by_uri(&watcher.user_uri)?
        .ok_or_else(|| SyncError::UserNotFoundError(watcher.user_uri.clone()))?;

    let client = client::Client::new_with_token(ctx.clone(), user.token)?;
    client.ensure_token_refreshed(&watcher.user_uri).await?;

    transfer::PlaylistTransfer::new(ctx.clone(), client)
        .try_transfer(&watcher)
        .await?;

    watcher_repo.update_watcher_last_sync_at(watcher.id, now)?;
    watcher_repo.update_watcher_next_sync_at(
        watcher.id,
        now.checked_add_signed(watcher.sync_interval.clone().into()).unwrap(),
    )?;

    Ok(())
}
