use crate::web::router::JWT_COOKIE;
use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use tower_cookies::Cookies;

pub async fn middleware<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> crate::Result<impl IntoResponse> {
    // If the user has a JWT (valid or not), redirect to the dashboard to let the auth middleware verify it
    if cookies.get(JWT_COOKIE).is_some() {
        return Ok(Redirect::to("/me").into_response());
    }

    Ok(next.run(req).await)
}
