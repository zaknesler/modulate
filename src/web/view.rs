use crate::model::{playlist::PlaylistType, watcher::Watcher};
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

impl DashboardTemplate {
    fn get_playlist_names(&self, watcher: &Watcher) -> (String, String) {
        (
            self.get_playlist_name(&watcher.from_playlist),
            self.get_playlist_name(&watcher.to_playlist),
        )
    }

    fn get_playlist_name(&self, playlist: &PlaylistType) -> String {
        match playlist {
            PlaylistType::Saved => playlist.to_string(),
            PlaylistType::WithId(id) => self
                .playlists
                .iter()
                .find(|playlist| &playlist.id.to_string() == id)
                .map(|playlist| playlist.name.clone())
                .unwrap_or_else(|| "(Unknown)".into()),
        }
    }
}
