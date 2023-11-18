use crate::{client::create_oauth_client, web::context::ApiContext};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use rspotify::clients::OAuthClient;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

pub fn router(ctx: Arc<ApiContext>) -> Router {
    Router::new()
        .route("/callback", get(handle_callback))
        .with_state(ctx)
}

#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
}

async fn handle_callback(
    Query(params): Query<CallbackParams>,
    State(ctx): State<Arc<ApiContext>>,
) -> crate::Result<impl IntoResponse> {
    let client = create_oauth_client(&ctx.config);

    client.request_token(&params.code).await?;

    let token = client.token.lock().await.unwrap();
    match token.as_ref().map(|token| &token.access_token) {
        Some(token) => ctx
            .db
            .get()?
            .execute("INSERT INTO tokens (token) VALUES (?)", &[token])?,
        None => return Ok(Json(json!({ "error": "no token" }))),
    };

    Ok(Json(json!({ "data": "finished" })))
}
