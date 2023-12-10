use super::{CSRF_COOKIE, JWT_COOKIE};
use crate::{
    api::client,
    context::AppContext,
    db::repo::user::UserRepo,
    web::{
        error::{WebError, WebResult},
        util::{
            cookie::unset_cookie,
            jwt::{self, JWT_EXPIRATION_DAYS},
        },
    },
};
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
) -> WebResult<impl IntoResponse> {
    // Ensure the state we get back from the API key is the value we set before the user was redirected
    if !cookies.get(CSRF_COOKIE).is_some_and(|cookie| cookie.value() == params.state) {
        return Err(WebError::CsrfInvalidError);
    }

    // Remove the CSRF cookie now that we've validated the response
    cookies.add(unset_cookie(CSRF_COOKIE));

    let client = client::Client::new(ctx.clone())?;

    let token = client.get_token_from_code(params.code).await?;
    client.set_token(token.clone())?;

    let user = client.current_user().await?;

    UserRepo::new(ctx.clone()).upsert_user_token(&user.uri, &token)?;

    let jwt = jwt::sign_jwt(ctx.config.web.jwt_secret.as_ref(), &user.uri.to_string())?;

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
