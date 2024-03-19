use self::error::SyncResult;
use crate::{
    api::client::{self, Client, WithToken},
    context::AppContext,
    db::{
        model::watcher::Watcher,
        repo::{transfer::TransferRepo, user::UserRepo, watcher::WatcherRepo},
    },
    sync::error::SyncError,
};
use chrono::{DateTime, Timelike, Utc};

pub mod error;
pub mod transfer;

pub async fn init(ctx: AppContext) -> SyncResult<()> {
    loop {
        let now = Utc::now();
        let next_update = now
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            .checked_add_signed(
                chrono::Duration::try_minutes(ctx.config.sync.check_interval_mins.into())
                    .expect("interval out of bounds"),
            )
            .unwrap();

        let time_until_next_update = tokio::time::Duration::from_millis(
            (next_update - now).num_milliseconds().try_into().unwrap(),
        );

        tracing::info!(
            "{:.0}s until next check",
            time_until_next_update.as_secs_f64(),
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
        let user = user_repo
            .find_user_by_uri(&watcher.user_uri)?
            .ok_or_else(|| SyncError::UserNotFound(watcher.user_uri.clone()))?;

        let (client, _) = client::Client::from_user_ensure_refreshed(ctx.clone(), user).await?;

        match sync_watcher(ctx.clone(), client, &watcher_repo, &watcher, now).await {
            Ok(_) => {
                // Set the next interval
                watcher_repo.update_watcher_next_sync_at(
                    watcher.id,
                    now.checked_add_signed(watcher.sync_interval.clone().into()).unwrap(),
                )?;
            }
            Err(err) => {
                // Don't kill worker thread if an individual sync task errored
                tracing::error!("Error when syncing watcher: {}", err);
                sentry::capture_error(&err);
            }
        }
    }

    tracing::info!("Synced");

    Ok(())
}

/// Sync a watcher and save the results to the transfer table.
pub async fn sync_watcher(
    ctx: AppContext,
    client: Client<WithToken>,
    watcher_repo: &WatcherRepo,
    watcher: &Watcher,
    now: DateTime<Utc>,
) -> SyncResult<u32> {
    let res = sync_watcher_inner(ctx.clone(), client, watcher_repo, watcher, &now).await;

    let num_tracks = *res.as_ref().unwrap_or(&0);

    // Only log if we've actually transferred tracks
    if num_tracks == 0 {
        return Ok(0);
    }

    // Save transfer result
    TransferRepo::new(ctx.clone()).log_transfer(
        watcher.id,
        res.as_ref().unwrap_or(&0),
        &res.as_ref().err(),
        now,
    )?;

    res
}

/// Sync a watcher and update the `last_sync_at` date
async fn sync_watcher_inner(
    ctx: AppContext,
    client: Client<WithToken>,
    watcher_repo: &WatcherRepo,
    watcher: &Watcher,
    now: &DateTime<Utc>,
) -> SyncResult<u32> {
    let num_tracks_transferred =
        transfer::PlaylistTransfer::new(ctx, client).try_transfer(watcher).await?;

    watcher_repo.update_watcher_last_sync_at(watcher.id, *now)?;

    Ok(num_tracks_transferred)
}
