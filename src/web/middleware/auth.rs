use crate::{context::AppContext, util::client, web::router::COOKIE_USER_ID};
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
use rspotify::{AuthCodeSpotify, Token};
use std::sync::Arc;
use tower_cookies::Cookies;

#[allow(dead_code)]
pub async fn middleware<B>(
    cookies: Cookies,
    State(ctx): State<Arc<AppContext>>,
    mut req: Request<B>,
    next: Next<B>,
) -> crate::Result<impl IntoResponse> {
    let user_id = cookies
        .get(COOKIE_USER_ID)
        .ok_or_else(|| crate::error::Error::UnauthorizedError)?;

    match try_create_auth_client(user_id.value(), ctx).await {
        Ok(client) => {
            req.extensions_mut().insert(client);
            Ok(next.run(req).await)
        }
        Err(_) => Err(crate::error::Error::UnauthorizedError),
    }
}

async fn try_create_auth_client(
    user_id: &str,
    ctx: Arc<AppContext>,
) -> crate::Result<AuthCodeSpotify> {
    let token: String = ctx
        .db
        .get()?
        .prepare("SELECT token FROM TOKENS WHERE user_id = ? LIMIT 1")?
        .query_row(&[user_id], |row| Ok(row.get(0)?))?;

    let token: Token = serde_json::from_str(&token)?;
    let client = client::create_from_token(token);

    // TODO: check that the token has not expired, and refresh if it has

    Ok(client)
}
