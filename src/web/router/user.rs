use std::sync::Arc;

use crate::web::context::ApiContext;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use rspotify::{model::UserId, prelude::*};
use serde_json::json;

pub fn router(ctx: Arc<ApiContext>) -> Router {
    Router::new()
        .route("/user/:username", get(get_user))
        .with_state(ctx)
}

async fn get_user(
    Path(username): Path<String>,
    State(ctx): State<Arc<ApiContext>>,
) -> crate::Result<impl IntoResponse> {
    let client = crate::client::create_anonymous_client(&ctx.config).await?;
    let user = client.user(UserId::from_id(username)?).await?;

    Ok(Json(json!({ "data": user })))
}
