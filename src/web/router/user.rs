use crate::{
    context::AppContext,
    web::{middleware::auth, view::DashboardTemplate},
};
use axum::{extract::State, middleware, response::IntoResponse, routing::get, Extension, Router};
use futures::TryStreamExt;
use rspotify::{model::PlaylistId, prelude::*, AuthCodeSpotify};
use std::sync::Arc;

pub fn router(ctx: Arc<AppContext>) -> Router {
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
    State(ctx): State<Arc<AppContext>>,
) -> crate::Result<impl IntoResponse> {
    let user = client.current_user().await?;
    let playlists = client
        .current_user_playlists()
        .try_collect::<Vec<_>>()
        .await?;
    let watched_playlist_id: Option<String> = ctx
        .db
        .get()?
        .prepare("SELECT playlist_id FROM watchers WHERE user_id = ? LIMIT 1")?
        .query_row(&[&user.id.to_string()], |row| Ok(row.get(0)?))
        .ok();

    let watched_playlist = if let Some(id) = watched_playlist_id {
        Some(
            client
                .user_playlist(
                    user.id.clone(),
                    Some(PlaylistId::from_id_or_uri(&id)?),
                    None,
                )
                .await?
                .name,
        )
    } else {
        None
    };

    Ok(DashboardTemplate {
        name: user.id.to_string().split(':').last().unwrap().to_owned(),
        watched_playlist,
        playlists,
    })
}
