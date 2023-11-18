use askama::Template;

#[derive(Template)]
#[template(path = "auth.html")]
pub struct AuthTemplate {
    pub url: String,
}

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate {
    pub name: String,
}
