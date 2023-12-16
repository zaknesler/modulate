use crate::{
    api,
    context::AppContext,
    db::{model::user::User, repo::user::UserRepo},
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

    let session = try_create_auth_session(ctx, user)
        .await
        .map_err(|_| WebError::UnauthorizedError)?;

    req.extensions_mut().insert(session);

    Ok(next.run(req).await)
}

async fn try_create_auth_session(ctx: AppContext, user: User) -> WebResult<session::Session> {
    let (client, user) = api::client::Client::from_user_ensure_refreshed(ctx, user.clone()).await?;

    Ok(session::Session { client, user })
}
