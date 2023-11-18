use super::view::AuthTemplate;
use crate::{client::create_oauth_client, context::AppContext};
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use std::sync::Arc;

mod auth;
mod user;

pub const COOKIE_TOKEN: &str = "token";

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", get(root))
        .with_state(ctx.clone())
        .merge(auth::router(ctx.clone()))
        .merge(user::router(ctx))
}

async fn root(State(ctx): State<Arc<AppContext>>) -> crate::Result<impl IntoResponse> {
    let client = create_oauth_client(&ctx.config);
    let url = client.get_authorize_url(true)?;

    Ok(AuthTemplate { url })
}
