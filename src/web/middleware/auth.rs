use crate::{
    api,
    context::AppContext,
    repo::user::UserRepo,
    util::jwt,
    web::{router::JWT_COOKIE, session},
    CONFIG,
};
use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use tower_cookies::{
    cookie::{
        time::{ext::NumericalDuration, OffsetDateTime},
        CookieBuilder,
    },
    Cookies,
};

pub async fn middleware(
    cookies: Cookies,
    State(ctx): State<AppContext>,
    mut req: Request<Body>,
    next: Next,
) -> crate::Result<impl IntoResponse> {
    let (user_id, token) = match cookies
        .get(JWT_COOKIE)
        .and_then(|cookie| jwt::verify_jwt(CONFIG.web.jwt_secret.as_ref(), cookie.value()).ok())
        .and_then(|user_id| {
            UserRepo::new(ctx.clone())
                .get_token_by_user_id(&user_id)
                .ok()
                .map(|token| (user_id, token))
        }) {
        Some(value) => value,
        None => {
            // Unset the JWT cookie if it isn't valid
            cookies.add(
                CookieBuilder::new(JWT_COOKIE, "")
                    .path("/")
                    .expires(OffsetDateTime::now_utc().checked_sub(1.days()))
                    .build(),
            );

            return Ok(Redirect::to("/").into_response());
        }
    };

    let session = try_create_auth_session(&user_id, &token, ctx)
        .await
        .map_err(|_| crate::error::Error::UnauthorizedError)?;

    req.extensions_mut().insert(session);

    Ok(next.run(req).await)
}

async fn try_create_auth_session(
    user_id: &str,
    token_str: &str,
    ctx: AppContext,
) -> crate::Result<session::Session> {
    let (client, token) =
        api::client::get_token_ensure_refreshed(user_id, &serde_json::from_str(token_str)?, ctx)
            .await?;

    Ok(session::Session {
        user_id: user_id.to_string(),
        token,
        client,
    })
}
