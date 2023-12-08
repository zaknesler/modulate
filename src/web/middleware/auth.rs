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
    CONFIG,
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
    let (user_uri, token) = match cookies
        .get(JWT_COOKIE)
        .and_then(|cookie| jwt::verify_jwt(CONFIG.web.jwt_secret.as_ref(), cookie.value()).ok())
        .and_then(|user_uri| {
            UserRepo::new(ctx.clone())
                .get_token_by_user_uri(&user_uri)
                .ok()
                .map(|token| (user_uri, token))
        }) {
        Some(value) => value,
        None => {
            // Unset the JWT cookie if it isn't valid
            cookies.add(unset_cookie(JWT_COOKIE));

            return Ok(Redirect::to("/").into_response());
        }
    };

    let session = try_create_auth_session(&user_uri, token, ctx)
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
    let client = api::client::Client::new_with_token(token.clone())?;

    // Ensure access token is refreshed
    client.ensure_token_refreshed(ctx, user_uri).await?;

    Ok(session::Session {
        user_uri: user_uri.to_string(),
        token,
        client,
    })
}
