use crate::{client, context::AppContext, web::router::COOKIE_TOKEN};
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
    let token = cookies
        .get(COOKIE_TOKEN)
        .ok_or_else(|| crate::error::Error::UnauthorizedError)?;

    match try_create_auth_client(token.value().to_owned(), ctx).await {
        Ok(client) => {
            req.extensions_mut().insert(client);
            Ok(next.run(req).await)
        }
        Err(_) => Err(crate::error::Error::UnauthorizedError),
    }
}

async fn try_create_auth_client(
    token: String,
    ctx: Arc<AppContext>,
) -> crate::Result<AuthCodeSpotify> {
    // let token: String = ctx
    //     .db
    //     .get()?
    //     .prepare("SELECT token FROM TOKENS LIMIT 1")?
    //     .query_row(params![], |row| Ok(row.get(0)?))?;

    let token: Token = serde_json::from_str(&token)?;
    let client = client::create_from_token(token);

    // TODO: check that the token has not expired, and refresh if it has

    Ok(client)
}
