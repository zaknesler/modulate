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
        let from_name = match &watcher.from_playlist {
            PlaylistType::Saved => watcher.from_playlist.to_string(),
            PlaylistType::WithId(id) => self
                .find_playlist_by_id(id)
                .map(|playlist| playlist.name.clone())
                .unwrap_or_else(|| "(Unknown)".into()),
        };

        let to_name = match &watcher.to_playlist {
            PlaylistType::Saved => watcher.to_playlist.to_string(),
            PlaylistType::WithId(id) => self
                .find_playlist_by_id(id)
                .map(|playlist| playlist.name.clone())
                .unwrap_or_else(|| "(Unknown)".into()),
        };

        (from_name, to_name)
    }

    fn find_playlist_by_id(&self, id: &str) -> Option<&SimplifiedPlaylist> {
        self.playlists
            .iter()
            .find(|playlist| playlist.id.to_string() == id)
    }
}
