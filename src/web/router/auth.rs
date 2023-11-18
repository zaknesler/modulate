use super::COOKIE_TOKEN;
use crate::{context::AppContext, util::client::create_oauth_client};
use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use rspotify::clients::OAuthClient;
use serde::Deserialize;
use std::sync::Arc;
use tower_cookies::{Cookie, Cookies};

pub fn router(ctx: Arc<AppContext>) -> Router {
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
    cookies: Cookies,
    State(ctx): State<Arc<AppContext>>,
) -> crate::Result<impl IntoResponse> {
    let client = create_oauth_client(&ctx.config);
    client.request_token(&params.code).await?;

    let token = client
        .token
        .lock()
        .await
        .unwrap()
        .as_ref()
        .and_then(|token| serde_json::to_string(token).ok())
        .ok_or_else(|| anyhow!("no token"))?;

    ctx.db
        .get()?
        .execute("INSERT INTO tokens (token) VALUES (?)", &[&token])?;

    cookies.add(Cookie::new(COOKIE_TOKEN, token));

    Ok(Redirect::to("/me"))
}
