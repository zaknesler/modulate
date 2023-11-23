use crate::{
    context::AppContext,
    model::playlist::PlaylistType,
    repo::watcher::WatcherRepo,
    sync::transfer,
    web::{middleware::auth, session},
};
use axum::{
    extract::{Form, Path, State},
    middleware,
    response::{IntoResponse, Redirect},
    routing::post,
    Extension, Router,
};
use serde::Deserialize;
use validator::Validate;

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/watchers", post(create_watcher))
        .route("/watchers/:id/sync", post(sync_watcher))
        .route("/watchers/:id/delete", post(delete_watcher))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

#[derive(Debug, Deserialize, Validate)]
struct CreateWatcherParams {
    #[validate(required)]
    playlist_from: Option<String>,

    #[validate(required)]
    playlist_to: Option<String>,

    should_remove: Option<String>,
}

async fn create_watcher(
    Extension(session): Extension<session::Session>,
    State(ctx): State<AppContext>,
    Form(data): Form<CreateWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    data.validate()?;

    let from = data.playlist_from.expect("validated");
    let to = data.playlist_to.expect("validated");

    if to == from {
        return Err(crate::error::Error::InvalidFormData(
            "cannot create watcher that transfers between the same playlist".into(),
        ));
    }

    WatcherRepo::new(ctx.clone()).create_watcher(
        &session.user_id,
        PlaylistType::from_value(&from),
        PlaylistType::from_value(&to),
        data.should_remove.is_some(),
    )?;

    Ok(Redirect::to("/me"))
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
    let watcher = repo.get_watcher_by_id_and_user(params.id, &session.user_id)?;

    repo.delete_watcher_by_user_and_playlists(
        &session.user_id,
        watcher.playlist_from,
        watcher.playlist_to,
    )?;

    Ok(Redirect::to("/me"))
}

async fn sync_watcher(
    Extension(session): Extension<session::Session>,
    State(ctx): State<AppContext>,
    Path(params): Path<ManageWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    let repo = WatcherRepo::new(ctx.clone());
    let watcher = repo.get_watcher_by_id_and_user(params.id, &session.user_id)?;

    transfer::PlaylistTransfer::new(ctx, session.client)
        .try_transfer(&watcher)
        .await?;

    Ok(Redirect::to("/me"))
}
