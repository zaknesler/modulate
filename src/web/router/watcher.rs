use crate::{
    context::AppContext, model::playlist::PlaylistType, repo::watcher::WatcherRepo, sync::transfer,
    web::middleware::auth,
};
use axum::{
    extract::{Form, Path, State},
    middleware,
    response::{IntoResponse, Redirect},
    routing::post,
    Extension, Router,
};
use rspotify::{prelude::*, AuthCodeSpotify};
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

#[derive(Deserialize, Validate)]
struct CreateWatcherParams {
    #[validate(required)]
    from_playlist: Option<String>,

    #[validate(required)]
    to_playlist: Option<String>,
}

async fn create_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
    Form(data): Form<CreateWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    data.validate()?;

    let from = data.from_playlist.expect("validated");
    let to = data.to_playlist.expect("validated");

    if to == from {
        return Err(crate::error::Error::InvalidFormData(
            "cannot create watcher that transfers between the same playlist".into(),
        ));
    }

    let user = client.current_user().await?;
    let from_playlist = PlaylistType::from_value(&from);
    let to_playlist = PlaylistType::from_value(&to);

    WatcherRepo::new(ctx.clone()).create_watcher(
        &user.id.to_string(),
        from_playlist,
        to_playlist,
    )?;

    Ok(Redirect::to("/me"))
}

#[derive(Deserialize)]
struct ManageWatcherParams {
    id: i64,
}

async fn delete_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
    Path(params): Path<ManageWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    let user = client.current_user().await?;

    let repo = WatcherRepo::new(ctx);
    let watcher = repo.get_watcher_by_id_and_user(params.id, &user.id.to_string())?;

    repo.delete_watcher(
        &user.id.to_string(),
        watcher.from_playlist,
        watcher.to_playlist,
    )?;

    Ok(Redirect::to("/me"))
}

async fn sync_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
    Path(params): Path<ManageWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    let user = client.current_user().await?;

    let repo = WatcherRepo::new(ctx.clone());
    let watcher = repo.get_watcher_by_id_and_user(params.id, &user.id.to_string())?;

    transfer::PlaylistTransfer::new(ctx, client)
        .transfer(watcher.from_playlist, watcher.to_playlist)
        .await?;

    Ok(Redirect::to("/me"))
}
