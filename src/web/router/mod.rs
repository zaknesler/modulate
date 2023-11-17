use super::{context::ApiContext, view::IndexTemplate};
use axum::{response::IntoResponse, routing::get, Router};

mod auth;
mod user;

pub fn router(ctx: ApiContext) -> Router {
    Router::new()
        .route("/", get(root))
        .merge(auth::router(ctx.clone()))
        .merge(user::router(ctx))
}

async fn root() -> crate::Result<impl IntoResponse> {
    Ok(IndexTemplate { name: "zak" })
}
