use crate::model::watcher::Watcher;
use askama::Template;
use rspotify::model::SimplifiedPlaylist;

#[derive(Template)]
#[template(path = "connect.html")]
pub struct ConnectTemplate {
    pub url: String,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub name: String,
    pub watchers: Vec<Watcher>,
    pub playlists: Vec<SimplifiedPlaylist>,
}
