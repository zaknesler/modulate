use crate::{
    context::AppContext,
    repo::user::UserRepo,
    util::{client, jwt},
    web::router::JWT_COOKIE,
    CONFIG,
};
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
use rspotify::AuthCodeSpotify;
use tower_cookies::Cookies;

pub async fn middleware<B>(
    cookies: Cookies,
    State(ctx): State<AppContext>,
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

async fn try_create_auth_client(jwt: &str, ctx: AppContext) -> crate::Result<AuthCodeSpotify> {
    let user_id = jwt::verify_jwt(CONFIG.web.jwt_secret.as_ref(), jwt)?;
    let token = UserRepo::new(ctx.clone()).get_token_by_user_id(&user_id)?;

    client::get_token_ensure_refreshed(user_id, &serde_json::from_str(&token)?, ctx).await
}
