use super::JWT_COOKIE;
use crate::{
    context::AppContext,
    repo::user::UserRepo,
    util::{client::create_oauth_client, jwt},
};
use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use rspotify::clients::{BaseClient, OAuthClient};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

pub fn router(ctx: AppContext) -> Router {
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
    State(ctx): State<AppContext>,
) -> crate::Result<impl IntoResponse> {
    let client = create_oauth_client(&ctx.config);
    client.request_token(&params.code).await?;

    let user_id = client.current_user().await?.id;

    let token = client
        .get_token()
        .lock()
        .await
        .unwrap()
        .as_ref()
        .and_then(|token| serde_json::to_string(token).ok())
        .ok_or_else(|| anyhow!("no token"))?;

    UserRepo::new(ctx.clone()).upsert_user_token(&user_id.to_string(), &token)?;

    let jwt = jwt::sign_jwt(&ctx.config.web.jwt_secret, user_id.to_string())?;
    cookies.add(Cookie::new(JWT_COOKIE, jwt));

    Ok(Redirect::to("/me"))
}
