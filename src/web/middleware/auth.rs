use crate::{
    context::AppContext,
    util::{client, jwt},
    web::router::JWT_COOKIE,
};
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
use rspotify::{clients::BaseClient, AuthCodeSpotify, Token};
use std::sync::Arc;
use tower_cookies::Cookies;

#[allow(dead_code)]
pub async fn middleware<B>(
    cookies: Cookies,
    State(ctx): State<Arc<AppContext>>,
    mut req: Request<B>,
    next: Next<B>,
) -> crate::Result<impl IntoResponse> {
    let jwt_cookie = cookies
        .get(JWT_COOKIE)
        .ok_or_else(|| crate::error::Error::UnauthorizedError)?;

    let client = try_create_auth_client(jwt_cookie.value(), ctx)
        .await
        .map_err(|_| crate::error::Error::UnauthorizedError)?;

    // Add Spotify OAuth client as extension to be accessed by any route that wishes to perform action on user's behalf
    req.extensions_mut().insert(client);

    Ok(next.run(req).await)
}

async fn try_create_auth_client(jwt: &str, ctx: Arc<AppContext>) -> crate::Result<AuthCodeSpotify> {
    let user_id = jwt::verify_jwt(&ctx.config.web.jwt_secret, jwt)?;

    let token: String = ctx
        .db
        .get()?
        .prepare("SELECT token FROM users WHERE user_id = ? LIMIT 1")?
        .query_row(&[&user_id], |row| Ok(row.get(0)?))?;

    let token: Token = serde_json::from_str(&token)?;
    let mut client = client::create_from_token(token.clone());

    let is_expired = client
        .get_token()
        .lock()
        .await
        .unwrap()
        .as_ref()
        .map(|token| token.is_expired())
        .is_some_and(|val| val);

    if is_expired {
        // Create new client with our credentials and add our current token
        client = client::create_oauth_client(&ctx.config);
        *client.token.lock().await.unwrap() = Some(token);

        // Request a new token
        let token = client
            .refetch_token()
            .await?
            .ok_or_else(|| anyhow::anyhow!("could not refetch token"))?;

        // Override the token in the client
        *client.get_token().lock().await.unwrap() = Some(token.clone());
        client.write_token_cache().await?;

        // Update the token in the database
        ctx.db
            .get()?
            .prepare("UPDATE users SET token = ? WHERE user_id = ?")?
            .execute(&[&serde_json::to_string(&token)?, &user_id])?;
    }

    // If we requested a new token, the client now has it
    Ok(client)
}
