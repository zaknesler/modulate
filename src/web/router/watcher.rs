use crate::{
    api::{self, id::UserId},
    context::AppContext,
    db::model::{playlist::PlaylistType, watcher::SyncInterval},
    db::repo::watcher::WatcherRepo,
    web::{
        error::{WebError, WebResult},
        middleware::auth,
        session,
    },
};
use axum::{
    extract::{Path, State},
    middleware,
    response::IntoResponse,
    routing::{delete, post},
    Extension, Json, Router,
};
use chrono::Utc;
use reqwest::StatusCode;
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
) -> WebResult<impl IntoResponse> {
    data.validate()?;

    let from = PlaylistType::try_from_value(&data.playlist_from)?;
    let to = PlaylistType::try_from_value(&data.playlist_to)?;

    if to == from {
        return Err(WebError::InvalidFormData(
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
        return Err(WebError::InvalidFormData(
            "A watcher with track removal enabled already exists for this playlist.".into(),
        ));
    }

    if data.should_remove && !existing_watchers.is_empty() {
        return Err(WebError::InvalidFormData(
            "A watcher already exists for this playlist. Disable track removal or remove the other watcher.".into(),
        ));
    }

    if let PlaylistType::Id(id) = &from {
        let user_id = UserId::parse_from_input(&session.user.user_uri)?;
        match api::util::check_playlist_editable(&session.client, id, &user_id).await {
            Ok(false) if data.should_remove => return Err(WebError::InvalidFormData(
                "You do not have permission to edit the source playlist. You must disable track removal.".into(),
            )),
            Ok(_) => {}
            Err(ref _err @ api::error::ClientError::ApiError { status, message: _ }) if status == StatusCode::BAD_GATEWAY => {
                // Spotify returns a 502 Bad Gateway error if the playlist could not be found
                return Err(WebError::InvalidFormData("Source playlist does not exist.".into()))
            },
            Err(err) => return Err(err.into()),
        };
    }

    repo.create_watcher(
        &session.user.user_uri,
        &from,
        &to,
        data.should_remove,
        data.sync_interval,
    )
    .map_err(|err| match err {
        crate::db::error::DbError::SQLiteError(
            ref _inner @ rusqlite::Error::SqliteFailure(ref err_code, _),
        ) if err_code.code == rusqlite::ErrorCode::ConstraintViolation => {
            WebError::InvalidFormData("Watcher already exists for these playlists.".into())
        }
        _ => err.into(),
    })?;

    Ok(Json(json!({ "success": true })))
}

#[derive(Deserialize)]
struct ManageWatcherParams {
    id: u32,
}

async fn delete_watcher(
    Extension(session): Extension<session::Session>,
    State(ctx): State<AppContext>,
    Path(params): Path<ManageWatcherParams>,
) -> WebResult<impl IntoResponse> {
    let repo = WatcherRepo::new(ctx);

    let watcher = match repo.get_watcher_by_id_and_user(params.id, &session.user.user_uri)? {
        Some(val) => val,
        None => return Err(WebError::NotFoundError),
    };

    repo.delete_watcher_by_user_and_playlists(
        &session.user.user_uri,
        &watcher.playlist_from,
        &watcher.playlist_to,
    )?;

    Ok(Json(json!({ "success": true })))
}

async fn sync_watcher(
    Extension(session): Extension<session::Session>,
    State(ctx): State<AppContext>,
    Path(params): Path<ManageWatcherParams>,
) -> WebResult<impl IntoResponse> {
    let watcher_repo = WatcherRepo::new(ctx.clone());

    let watcher =
        match watcher_repo.get_watcher_by_id_and_user(params.id, &session.user.user_uri)? {
            Some(val) => val,
            None => return Err(WebError::NotFoundError),
        };

    let count =
        crate::sync::sync_watcher(ctx, session.client, &watcher_repo, &watcher, Utc::now()).await?;

    Ok(Json(json!({
        "success": true,
        "num_tracks_transferred": count
    })))
}
