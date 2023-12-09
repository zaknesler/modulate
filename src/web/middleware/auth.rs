use crate::{
    api::{self, token::Token},
    context::AppContext,
    db::repo::user::UserRepo,
    web::util::{cookie::unset_cookie, jwt},
    web::{
        error::{WebError, WebResult},
        router::JWT_COOKIE,
        session,
    },
};
use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use tower_cookies::Cookies;

pub async fn middleware(
    cookies: Cookies,
    State(ctx): State<AppContext>,
    mut req: Request<Body>,
    next: Next,
) -> WebResult<impl IntoResponse> {
    let user = match cookies
        .get(JWT_COOKIE)
        .and_then(|cookie| jwt::verify_jwt(ctx.config.web.jwt_secret.as_ref(), cookie.value()).ok())
        .and_then(|user_uri| UserRepo::new(ctx.clone()).find_user_by_uri(&user_uri).ok())
        .flatten()
    {
        Some(value) => value,
        None => {
            // Unset the JWT cookie if it isn't valid
            cookies.add(unset_cookie(JWT_COOKIE));

            return Ok(Redirect::to("/").into_response());
        }
    };

    let session = try_create_auth_session(&user.user_uri, user.token, ctx)
        .await
        .map_err(|_| WebError::UnauthorizedError)?;

    req.extensions_mut().insert(session);

    Ok(next.run(req).await)
}

async fn try_create_auth_session(
    user_uri: &str,
    token: Token,
    ctx: AppContext,
) -> WebResult<session::Session> {
    let client = api::client::Client::new_with_token(ctx, token.clone())?;

    // Ensure access token is refreshed
    client.ensure_token_refreshed(user_uri).await?;

    Ok(session::Session {
        user_uri: user_uri.to_string(),
        token,
        client,
    })
}
