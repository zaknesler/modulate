use crate::web::{context::ApiContext, middleware::auth};
use axum::{middleware, response::IntoResponse, routing::get, Extension, Json, Router};
use rspotify::{prelude::*, Token};
use serde_json::json;
use std::sync::Arc;

pub fn router(ctx: Arc<ApiContext>) -> Router {
    Router::new()
        .route("/me", get(get_current_user))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

async fn get_current_user(Extension(token): Extension<Token>) -> crate::Result<impl IntoResponse> {
    let client = crate::client::create_from_token(&token);
    let user = client.current_user().await?;

    Ok(Json(json!({ "data": user })))
}
