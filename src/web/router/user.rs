use super::JWT_COOKIE;
use crate::{
    context::AppContext,
    repo::{user::UserRepo, watcher::WatcherRepo},
    web::{middleware::auth, session, view::DashboardTemplate},
};
use axum::{
    extract::State,
    middleware,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Extension, Router,
};
use futures::TryStreamExt;
use rspotify::prelude::*;
use tower_cookies::{
    cookie::{
        time::{ext::NumericalDuration, OffsetDateTime},
        CookieBuilder,
    },
    Cookies,
};

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/me", get(get_current_user_dashboard))
        .route("/me/delete", post(delete_current_user))
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
    let user = session.client.current_user().await?;

    let watchers = WatcherRepo::new(ctx.clone()).get_all_watchers_by_user(&user.id.to_string())?;
    let playlists = session
        .client
        .current_user_playlists()
        .try_collect::<Vec<_>>()
        .await?;

    Ok(DashboardTemplate {
        name: user.id.id().into(),
        watchers,
        playlists,
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
    cookies.add(
        CookieBuilder::new(JWT_COOKIE, "")
            .path("/")
            .expires(OffsetDateTime::now_utc().checked_sub(1.days()))
            .finish(),
    );

    Ok(Redirect::to("/"))
}
