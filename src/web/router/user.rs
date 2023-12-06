use super::JWT_COOKIE;
use crate::{
    api,
    context::AppContext,
    repo::{user::UserRepo, watcher::WatcherRepo},
    util::cookie::unset_cookie,
    web::{
        middleware::auth,
        session,
        view::{DashboardTemplate, DisplayPlaylist},
    },
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
        // .route("/me/demo", get(demo))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

async fn get_current_user_dashboard(
    Extension(session): Extension<session::Session>,
    State(ctx): State<AppContext>,
) -> crate::Result<impl IntoResponse> {
    let user = session.client.get_current_user().await?;

    let watchers = WatcherRepo::new(ctx.clone()).get_watchers_by_user(&user.id)?;

    // Get all playlists that belong to the user
    let user_playlists = session
        .client
        .current_user_playlists()
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .map(|item| item.into())
        .collect::<Vec<DisplayPlaylist>>();
    let user_playlist_ids = user_playlists
        .iter()
        .filter_map(|playlist| playlist.uri.as_ref().map(|uri| uri.to_owned()))
        .collect::<HashSet<_>>();

    // Fetch the details of the playlists that the user does not own
    let missing_playlist_ids = watchers
        .iter()
        .flat_map(|watcher| vec![&watcher.playlist_from, &watcher.playlist_to])
        .filter_map(|playlist| match playlist {
            crate::model::playlist::PlaylistType::Uri(uri) => Some(uri.to_owned()),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let missing_playlists = api::util::get_playlists_by_ids(
        session.client,
        missing_playlist_ids.difference(&user_playlist_ids),
    )
    .await?
    .into_iter()
    .map(|item| item.into())
    .collect::<Vec<DisplayPlaylist>>();

    // Combine the playlists that either belong to the user or are referenced by a watcher, in display format
    let all_playlists = user_playlists
        .iter()
        .cloned()
        .chain(missing_playlists.iter().cloned())
        .collect();

    Ok(DashboardTemplate {
        name: user.display_name,
        watchers,
        user_playlists,
        all_playlists,
    })
}

async fn delete_current_user(
    Extension(session): Extension<session::Session>,
    cookies: Cookies,
    State(ctx): State<AppContext>,
) -> crate::Result<impl IntoResponse> {
    // Delete all user's watchers and then the user
    WatcherRepo::new(ctx.clone()).delete_all_watchers_by_user(&session.user_id)?;
    UserRepo::new(ctx).delete_user_by_id(&session.user_id)?;

    // Unset the JWT cookie
    cookies.add(unset_cookie(JWT_COOKIE));

    Ok(Json(json!({ "success": true })))
}

// async fn demo(Extension(session): Extension<session::Session>) -> crate::Result<impl IntoResponse> {
//     let playlist = session.client2.get_playlist_partial("5qgLEa0o3k51FH78jSp50D").await?;

//     Ok(Json(json!({ "data": playlist })))
// }
