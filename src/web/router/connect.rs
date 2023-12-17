use super::{CSRF_COOKIE, JWT_COOKIE};
use crate::{
    api::client::Client,
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

    // Use the code we got from Spotify to create a valid token
    let token = Client::new(ctx.clone())?.get_token_from_code(params.code).await?;

    // Initialize a new client to be able to send authenticated API requests
    let client = Client::new_with_token(ctx.clone(), token.clone())?;

    // Fetch the user and save the user/token to the database
    let user = client.current_user().await?;
    UserRepo::new(ctx.clone()).upsert_user_token(&user.id.uri(), &token)?;

    // Create a JWT to allow the user to authenticate
    let jwt = jwt::sign_jwt(
        ctx.config.web.jwt_secret.as_ref(),
        &user.id.uri().to_string(),
    )?;

    // Set JWT cookie
    cookies.add(
        CookieBuilder::new(JWT_COOKIE, jwt)
            .path("/")
            .expires(OffsetDateTime::now_utc().checked_add(JWT_EXPIRATION_DAYS.days()))
            .http_only(true)
            .same_site(tower_cookies::cookie::SameSite::Lax)
            .build(),
    );

    Ok(Redirect::to("/me"))
}
