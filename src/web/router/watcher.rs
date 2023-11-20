use crate::{
    context::AppContext, model::playlist::PlaylistType, repo::watcher::WatcherRepo, sync::transfer,
    web::middleware::auth,
};
use axum::{
    extract::{Form, State},
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
        .route("/watchers/sync", post(sync_watcher))
        .route("/watchers/delete", post(delete_watcher))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

#[derive(Deserialize, Validate)]
struct ManageWatcherParams {
    #[validate(required)]
    from_playlist: Option<String>,

    #[validate(required)]
    to_playlist: Option<String>,
}

async fn create_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
    Form(data): Form<ManageWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    data.validate()?;

    let user = client.current_user().await?;
    let from_playlist: PlaylistType = data.from_playlist.expect("validated").try_into()?;
    let to_playlist: PlaylistType = data.to_playlist.expect("validated").try_into()?;

    WatcherRepo::new(ctx.clone()).create_watcher(
        &user.id.to_string(),
        from_playlist,
        to_playlist,
    )?;

    Ok(Redirect::to("/me"))
}

async fn delete_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
    Form(data): Form<ManageWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    data.validate()?;

    let user = client.current_user().await?;
    let from_playlist: PlaylistType = data.from_playlist.expect("validated").try_into()?;
    let to_playlist: PlaylistType = data.to_playlist.expect("validated").try_into()?;

    WatcherRepo::new(ctx.clone()).delete_watcher(
        &user.id.to_string(),
        from_playlist,
        to_playlist,
    )?;

    Ok(Redirect::to("/me"))
}

async fn sync_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
    Form(data): Form<ManageWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    data.validate()?;

    let user = client.current_user().await?;
    let from_playlist: PlaylistType = data.from_playlist.expect("validated").try_into()?;
    let to_playlist: PlaylistType = data.to_playlist.expect("validated").try_into()?;

    let watchers = WatcherRepo::new(ctx.clone()).get_all_watchers_by_user(&user.id.to_string())?;
    if !watchers.is_empty() {
        transfer::PlaylistTransfer::new(ctx, client)
            .transfer(from_playlist, to_playlist)
            .await?;
    }

    Ok(Redirect::to("/me"))
}
