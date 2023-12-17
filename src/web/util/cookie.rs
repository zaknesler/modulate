use tower_cookies::{
    cookie::{
        time::{ext::NumericalDuration, OffsetDateTime},
        CookieBuilder,
    },
    Cookie,
};

pub fn unset_cookie<'a>(key: &'a str) -> Cookie<'a> {
    CookieBuilder::new(key, "")
        .path("/")
        .expires(OffsetDateTime::now_utc().checked_sub(1.days()))
        .http_only(true)
        .same_site(tower_cookies::cookie::SameSite::Lax)
        .build()
}
