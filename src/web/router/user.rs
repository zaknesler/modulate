use crate::{
    context::AppContext,
    model::watcher::Watcher,
    web::{middleware::auth, view::DashboardTemplate},
};
use axum::{extract::State, middleware, response::IntoResponse, routing::get, Extension, Router};
use futures::TryStreamExt;
use rspotify::{prelude::*, AuthCodeSpotify};
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
    let watcher = ctx
        .db
        .get()?
        .prepare("SELECT playlist_id FROM watchers WHERE user_id = ? LIMIT 1")?
        .query_row(&[&user.id.to_string()], |row| {
            Ok(Watcher {
                user_id: user.id.to_string(),
                playlist_id: row.get(0)?,
            })
        })
        .ok();

    Ok(DashboardTemplate {
        name: user.id.to_string().split(':').last().unwrap().to_owned(),
        watcher,
        playlists,
    })
}
