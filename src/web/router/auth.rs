use crate::web::context::ApiContext;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde_json::json;

pub fn router(ctx: ApiContext) -> Router {
    Router::new()
        .route("/auth", post(create_session))
        .with_state(ctx)
}

async fn create_session(State(ctx): State<ApiContext>) -> crate::Result<impl IntoResponse> {
    Ok(Json(json!({ "data": "creating" })))
}
