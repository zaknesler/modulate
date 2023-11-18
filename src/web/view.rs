use askama::Template;
use rspotify::model::SimplifiedPlaylist;

#[derive(Template)]
#[template(path = "auth.html")]
pub struct AuthTemplate {
    pub url: String,
}

#[derive(Template)]
#[template(path = "watcher.html")]
pub struct WatcherTemplate {
    pub name: String,
    pub playlists: Vec<SimplifiedPlaylist>,
}
