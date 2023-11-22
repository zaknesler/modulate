use crate::{
    context::AppContext,
    repo::user::UserRepo,
    util::{client, jwt},
    web::{router::JWT_COOKIE, session},
    CONFIG,
};
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
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

    let session = try_create_auth_session(jwt_cookie.value(), ctx)
        .await
        .map_err(|_| crate::error::Error::UnauthorizedError)?;

    req.extensions_mut().insert(session);

    Ok(next.run(req).await)
}

async fn try_create_auth_session(jwt: &str, ctx: AppContext) -> crate::Result<session::Session> {
    let user_id = jwt::verify_jwt(CONFIG.web.jwt_secret.as_ref(), jwt)?;
    let token_str = &UserRepo::new(ctx.clone()).get_token_by_user_id(&user_id)?;

    let (client, token) = client::get_token_ensure_refreshed(
        user_id.clone(),
        &serde_json::from_str(&token_str)?,
        ctx,
    )
    .await?;

    Ok(session::Session {
        user_id,
        token,
        client,
    })
}
