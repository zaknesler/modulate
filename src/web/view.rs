use askama::Template;
use rspotify::model::SimplifiedPlaylist;

use crate::model::watcher::Watcher;

#[derive(Template)]
#[template(path = "auth.html")]
pub struct AuthTemplate {
    pub url: String,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub name: String,
    pub watcher: Option<Watcher>,
    pub playlists: Vec<SimplifiedPlaylist>,
}
