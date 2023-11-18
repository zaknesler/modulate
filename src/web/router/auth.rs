use crate::{client::create_oauth_client, web::context::ApiContext};
use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use r2d2_sqlite::rusqlite::params;
use rspotify::clients::OAuthClient;
use serde::Deserialize;
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
    match token
        .as_ref()
        .map(|token| serde_json::to_string(token).ok())
        .flatten()
    {
        Some(token) => ctx
            .db
            .get()?
            .execute("INSERT INTO tokens (token) VALUES (?)", params![token])?,
        None => return Err(anyhow!("no token").into()),
    };

    Ok(Redirect::to("/me"))
}
