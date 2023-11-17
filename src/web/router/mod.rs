use super::context::ApiContext;
use crate::error::SpotifyResult;
use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

mod user;

pub fn router(ctx: ApiContext) -> Router {
    Router::new().route("/", get(root)).merge(user::router(ctx))
}

async fn root() -> SpotifyResult<impl IntoResponse> {
    Ok(Json(json!({ "message": "Hello, World!" })))
}
