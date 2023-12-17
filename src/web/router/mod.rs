use super::{error::WebResult, middleware::guest, view::ConnectTemplate};
use crate::{api::client, context::AppContext};
use axum::{extract::State, middleware, response::IntoResponse, routing::get, Router};
use tower_cookies::{
    cookie::{
        time::{Duration, OffsetDateTime},
        CookieBuilder,
    },
    Cookies,
};

mod connect;
mod user;
mod watcher;

pub const JWT_COOKIE: &str = "modulate_jwt";
pub const CSRF_COOKIE: &str = "modulate_csrf";

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/", get(root))
        .route_layer(middleware::from_fn(guest::middleware))
        .with_state(ctx.clone())
        .merge(connect::router(ctx.clone()))
        .merge(watcher::router(ctx.clone()))
        .merge(user::router(ctx))
}

async fn root(State(ctx): State<AppContext>, cookies: Cookies) -> WebResult<impl IntoResponse> {
    let (url, csrf) = client::Client::new(ctx)?.new_authorize_url();

    // Set CSRF cookie to verify once user is redirected back
    cookies.add(
        CookieBuilder::new(CSRF_COOKIE, csrf.secret().clone())
            .path("/")
            .expires(OffsetDateTime::now_utc().checked_add(Duration::hours(1)))
            .http_only(true)
            .same_site(tower_cookies::cookie::SameSite::Lax)
            .build(),
    );

    Ok(ConnectTemplate {
        url: url.to_string(),
    })
}
