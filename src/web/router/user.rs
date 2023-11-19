use crate::{
    context::AppContext,
    repo::watcher::WatcherRepo,
    web::{middleware::auth, view::DashboardTemplate},
};
use anyhow::anyhow;
use axum::{
    extract::State,
    middleware,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Extension, Router,
};
use futures::TryStreamExt;
use rspotify::{model::PlaylistId, prelude::*, AuthCodeSpotify};

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/me", get(get_dashboard))
        .route("/sync", post(sync_playlist))
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

    // Fetch the details about the watched playlist if one exists
    let watched_playlist = match WatcherRepo::new(ctx.clone())
        .get_watched_playlist_id_by_user_id(&user.id.to_string())?
    {
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

    // If the user doesn't have a watcher configured, fetch their playlists to let them create one
    let playlists = match watched_playlist {
        Some(_) => vec![],
        None => {
            client
                .current_user_playlists()
                .try_collect::<Vec<_>>()
                .await?
        }
    };

    Ok(DashboardTemplate {
        name: user.id.to_string().split(':').last().unwrap().to_owned(),
        watched_playlist,
        playlists,
    })
}

async fn sync_playlist(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
) -> crate::Result<impl IntoResponse> {
    let user_id = client.current_user().await?.id.to_string();
    let playlist_id = WatcherRepo::new(ctx.clone())
        .get_watched_playlist_id_by_user_id(&user_id)?
        .ok_or_else(|| anyhow!("no watched playlist"))?;
    let token = client
        .get_token()
        .lock()
        .await
        .unwrap()
        .clone()
        .ok_or_else(|| anyhow!("no token"))?;

    crate::sync::util::sync_user_playlist(&user_id, &playlist_id, &token, ctx).await?;

    Ok(Redirect::to("/me"))
}
