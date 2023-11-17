use crate::client::create_oauth_client;

use super::{context::ApiContext, view::IndexTemplate};
use axum::{extract::State, response::IntoResponse, routing::get, Router};

mod auth;
mod user;

pub fn router(ctx: ApiContext) -> Router {
    Router::new()
        .route("/", get(root))
        .with_state(ctx.clone())
        .merge(auth::router(ctx.clone()))
        .merge(user::router(ctx))
}

async fn root(State(ctx): State<ApiContext>) -> crate::Result<impl IntoResponse> {
    let client = create_oauth_client(&ctx.config);
    let auth_url = client.get_authorize_url(true)?;

    Ok(IndexTemplate { auth_url })
}
