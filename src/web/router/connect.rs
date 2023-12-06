use super::{CSRF_COOKIE, JWT_COOKIE};
use crate::{
    api::client2,
    context::AppContext,
    repo::user::UserRepo,
    util::jwt::{self, JWT_EXPIRATION_DAYS},
    CONFIG,
};
use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::Deserialize;
use tower_cookies::{
    cookie::{
        time::{ext::NumericalDuration, OffsetDateTime},
        CookieBuilder,
    },
    Cookies,
};

pub fn router(ctx: AppContext) -> Router {
    Router::new().route("/callback", get(handle_callback)).with_state(ctx)
}

#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

async fn handle_callback(
    Query(params): Query<CallbackParams>,
    cookies: Cookies,
    State(ctx): State<AppContext>,
) -> crate::Result<impl IntoResponse> {
    let csrf = cookies.get(CSRF_COOKIE).ok_or_else(|| anyhow!("missing csrf cookie"))?;

    if csrf.value() != params.state {
        return Err(anyhow!("invalid csrf token").into());
    }

    let client = client2::Client::new()?;

    let token = client.request_token(params.code).await?;
    client.set_token(token.clone())?;

    let user = client.me().await?;

    UserRepo::new(ctx.clone()).upsert_user_token(&user.uri, &token)?;

    let jwt = jwt::sign_jwt(CONFIG.web.jwt_secret.as_ref(), &user.uri.to_string())?;

    cookies.add(
        CookieBuilder::new(JWT_COOKIE, jwt)
            .path("/")
            .expires(OffsetDateTime::now_utc().checked_add(JWT_EXPIRATION_DAYS.days()))
            .http_only(true)
            .same_site(tower_cookies::cookie::SameSite::Strict)
            .build(),
    );

    Ok(Redirect::to("/me"))
}
