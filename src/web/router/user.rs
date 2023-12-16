use super::JWT_COOKIE;
use crate::{
    api::{self},
    context::AppContext,
    db::repo::{user::UserRepo, watcher::WatcherRepo},
    web::util::cookie::unset_cookie,
    web::{error::WebResult, middleware::auth, session, view::DashboardTemplate},
};
use axum::{
    extract::State,
    middleware,
    response::IntoResponse,
    routing::{delete, get},
    Extension, Json, Router,
};
use serde_json::json;
use std::collections::HashSet;
use tower_cookies::Cookies;

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/me", get(get_current_user_dashboard))
        .route("/me", delete(delete_current_user))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

async fn get_current_user_dashboard(
    Extension(session): Extension<session::Session>,
    State(ctx): State<AppContext>,
) -> WebResult<impl IntoResponse> {
    let user = session.client.current_user().await?;

    let watchers = WatcherRepo::new(ctx.clone()).get_watchers_by_user(&user.id.uri())?;

    // Get all playlists that belong to the user
    let user_playlists = session.client.current_user_playlists().await?;
    let user_playlist_ids = user_playlists
        .iter()
        .map(|playlist| playlist.id.clone())
        .collect::<HashSet<_>>();

    // Fetch the details of the playlists that the user does not own
    let missing_playlist_ids = watchers
        .iter()
        .flat_map(|watcher| vec![&watcher.playlist_from, &watcher.playlist_to])
        .filter_map(|playlist| match playlist {
            crate::db::model::playlist::PlaylistType::Id(id) => Some(id.to_owned()),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let missing_playlists = api::util::get_playlists_by_ids(
        session.client,
        missing_playlist_ids.difference(&user_playlist_ids),
    )
    .await?;

    Ok(DashboardTemplate {
        config: ctx.config,
        name: user.display_name,
        watchers,
        user_playlists: user_playlists
            .iter()
            .cloned()
            .map(|playlist| playlist.into())
            .collect::<Vec<_>>(),
        all_playlists: user_playlists
            .iter()
            .cloned()
            .chain(missing_playlists.iter().cloned())
            .map(|playlist| playlist.into())
            .collect::<Vec<_>>(),
    })
}

async fn delete_current_user(
    Extension(session): Extension<session::Session>,
    cookies: Cookies,
    State(ctx): State<AppContext>,
) -> WebResult<impl IntoResponse> {
    // Delete all user's watchers and then the user
    WatcherRepo::new(ctx.clone()).delete_all_watchers_by_user(&session.user.user_uri)?;
    UserRepo::new(ctx).delete_user_by_uri(&session.user.user_uri)?;

    // Unset the JWT cookie
    cookies.add(unset_cookie(JWT_COOKIE));

    Ok(Json(json!({ "success": true })))
}
