use super::{middleware::guest, view::ConnectTemplate};
use crate::{api::client::create_oauth_client, context::AppContext};
use axum::{middleware, response::IntoResponse, routing::get, Router};

mod connect;
mod user;
mod watcher;

pub const JWT_COOKIE: &str = "modulate_jwt";

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/", get(root))
        .route_layer(middleware::from_fn(guest::middleware))
        .with_state(ctx.clone())
        .merge(connect::router(ctx.clone()))
        .merge(watcher::router(ctx.clone()))
        .merge(user::router(ctx))
}

async fn root() -> crate::Result<impl IntoResponse> {
    let client = create_oauth_client();
    let url = client.get_authorize_url(true)?;

    Ok(ConnectTemplate { url })
}
