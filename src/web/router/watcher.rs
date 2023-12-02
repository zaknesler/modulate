use crate::{
    context::AppContext,
    model::{playlist::PlaylistType, watcher::SyncInterval},
    repo::watcher::WatcherRepo,
    sync::transfer,
    web::{middleware::auth, session},
};
use axum::{
    extract::{Path, State},
    middleware,
    response::IntoResponse,
    routing::{delete, post},
    Extension, Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/watchers", post(create_watcher))
        .route("/watchers/:id", delete(delete_watcher))
        .route("/watchers/:id/sync", post(sync_watcher))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

#[derive(Debug, Deserialize, Validate)]
struct CreateWatcherParams {
    playlist_from: String,
    playlist_to: String,
    should_remove: bool,
    sync_interval: SyncInterval,
}

async fn create_watcher(
    Extension(session): Extension<session::Session>,
    State(ctx): State<AppContext>,
    Json(data): Json<CreateWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    data.validate()?;

    let from = PlaylistType::try_from_value(&data.playlist_from)?;
    let to = PlaylistType::try_from_value(&data.playlist_to)?;

    if to == from {
        return Err(crate::error::Error::InvalidFormData(
            "Cannot create watcher that transfers between the same playlist.".into(),
        ));
    }

    let repo = WatcherRepo::new(ctx.clone());

    let existing_watchers = repo.get_watchers_for_playlist(&from)?;
    let existing_mutable_watchers = existing_watchers
        .iter()
        .filter(|watcher| watcher.should_remove)
        .collect::<Vec<_>>();

    if !existing_mutable_watchers.is_empty() {
        return Err(crate::error::Error::InvalidFormData(
            "Cannot create watcher as one already exists for this playlist with track removal enabled.".into(),
        ));
    } else if data.should_remove && !existing_watchers.is_empty() {
        return Err(crate::error::Error::InvalidFormData(
            "Cannot create watcher with track removal enabled as one already exists for this playlist.".into(),
        ));
    }

    repo.create_watcher(
        &session.user_id,
        &from,
        &to,
        data.should_remove,
        data.sync_interval,
    )?;

    Ok(Json(json!({ "success": true })))
}

#[derive(Deserialize)]
struct ManageWatcherParams {
    id: i64,
}

async fn delete_watcher(
    Extension(session): Extension<session::Session>,
    State(ctx): State<AppContext>,
    Path(params): Path<ManageWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    let repo = WatcherRepo::new(ctx);

    let watcher = match repo.get_watcher_by_id_and_user(params.id, &session.user_id)? {
        Some(val) => val,
        None => return Err(crate::error::Error::NotFoundError),
    };

    repo.delete_watcher_by_user_and_playlists(
        &session.user_id,
        &watcher.playlist_from,
        &watcher.playlist_to,
    )?;

    Ok(Json(json!({ "success": true })))
}

async fn sync_watcher(
    Extension(session): Extension<session::Session>,
    State(ctx): State<AppContext>,
    Path(params): Path<ManageWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    let repo = WatcherRepo::new(ctx.clone());

    let watcher = match repo.get_watcher_by_id_and_user(params.id, &session.user_id)? {
        Some(val) => val,
        None => return Err(crate::error::Error::NotFoundError),
    };

    transfer::PlaylistTransfer::new(ctx, session.client)
        .try_transfer(&watcher)
        .await?;

    Ok(Json(json!({ "success": true })))
}
