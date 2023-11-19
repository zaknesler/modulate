use askama::Template;
use rspotify::model::SimplifiedPlaylist;

#[derive(Template)]
#[template(path = "auth.html")]
pub struct AuthTemplate {
    pub url: String,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub name: String,
    pub watched_playlist: Option<String>,
    pub playlists: Vec<SimplifiedPlaylist>,
}
