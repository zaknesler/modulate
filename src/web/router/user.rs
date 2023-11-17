use crate::{error::SpotifyResult, web::context::ApiContext};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use rspotify::{model::UserId, prelude::*};
use serde_json::json;

pub fn router(ctx: ApiContext) -> Router {
    Router::new()
        .route("/user/:username", get(get_user))
        .with_state(ctx)
}

async fn get_user(
    Path(username): Path<String>,
    State(ctx): State<ApiContext>,
) -> SpotifyResult<impl IntoResponse> {
    let client = crate::client::create_client(&ctx.config).await?;
    let user = client.user(UserId::from_id(username)?).await?;

    Ok(Json(json!({ "data": user })))
}
