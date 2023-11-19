use crate::{
    context::AppContext,
    repo::watcher::WatcherRepo,
    web::{middleware::auth, view::DashboardTemplate},
};
use axum::{extract::State, middleware, response::IntoResponse, routing::get, Extension, Router};
use futures::TryStreamExt;
use rspotify::{model::PlaylistId, prelude::*, AuthCodeSpotify};

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/me", get(get_dashboard))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

async fn get_dashboard(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
) -> crate::Result<impl IntoResponse> {
    let user = client.current_user().await?;
    let playlists = client
        .current_user_playlists()
        .try_collect::<Vec<_>>()
        .await?;
    let watched_playlist_id =
        WatcherRepo::new(ctx.clone()).get_watched_playlist_id_by_user_id(&user.id.to_string())?;

    let watched_playlist = match watched_playlist_id {
        Some(id) => Some(
            client
                .user_playlist(
                    user.id.clone(),
                    Some(PlaylistId::from_id_or_uri(&id)?),
                    None,
                )
                .await?
                .name,
        ),
        None => None,
    };

    Ok(DashboardTemplate {
        name: user.id.to_string().split(':').last().unwrap().to_owned(),
        watched_playlist,
        playlists,
    })
}
