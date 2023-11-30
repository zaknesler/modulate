use crate::{
    constant::{SPOTIFY_EXTERNAL_URL_KEY, SPOTIFY_LIKED_TRACKS_URL},
    model::{playlist::PlaylistType, watcher::Watcher},
};
use askama::Template;
use rspotify::model::SimplifiedPlaylist;

#[derive(Template)]
#[template(path = "connect.html")]
pub struct ConnectTemplate {
    pub url: String,
}

#[derive(Debug, Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub name: String,
    pub watchers: Vec<Watcher>,
    pub playlists: Vec<SimplifiedPlaylist>,
}

#[derive(Debug)]
struct PlaylistDisplayData {
    pub kind: PlaylistType,
    pub display_name: String,
    pub image_url: Option<String>,
    pub spotify_url: String,
}

impl DashboardTemplate {
    fn get_mapped_display_data(
        &self,
        watcher: &Watcher,
    ) -> (Option<PlaylistDisplayData>, Option<PlaylistDisplayData>) {
        (
            self.map_display_data(&watcher.playlist_from),
            self.map_display_data(&watcher.playlist_to),
        )
    }

    fn map_display_data(&self, playlist: &PlaylistType) -> Option<PlaylistDisplayData> {
        match playlist {
            PlaylistType::Saved => Some(PlaylistDisplayData {
                kind: playlist.clone(),
                display_name: playlist.to_string(),
                image_url: None,
                spotify_url: SPOTIFY_LIKED_TRACKS_URL.into(),
            }),
            PlaylistType::WithId(id) => {
                self.playlists.iter().find(|data| data.id.to_string() == *id).map(|data| {
                    PlaylistDisplayData {
                        kind: playlist.clone(),
                        display_name: data.name.clone(),
                        image_url: match data.images.len() {
                            1 => data.images.first().map(|image| image.url.clone()),
                            2.. => data
                                .images
                                .iter()
                                .filter(|image| image.width.is_some())
                                .min_by(|a, b| a.width.cmp(&b.width))
                                .map(|image| image.url.clone()),
                            _ => None,
                        },
                        spotify_url: data
                            .external_urls
                            .get(SPOTIFY_EXTERNAL_URL_KEY)
                            .expect("should include spotify url")
                            .clone(),
                    }
                })
            }
        }
    }
}
