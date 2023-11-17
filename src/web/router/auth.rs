use crate::{client::create_oauth_client, web::context::ApiContext};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures::stream::TryStreamExt;
use rspotify::clients::OAuthClient;
use serde::Deserialize;
use serde_json::json;

pub fn router(ctx: ApiContext) -> Router {
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
    State(ctx): State<ApiContext>,
) -> crate::Result<impl IntoResponse> {
    let client = create_oauth_client(&ctx.config);

    client.request_token(&params.code).await?;

    let user = client.current_user_saved_tracks(None).try_next().await?;

    Ok(Json(json!({ "data": user })))
}
