use crate::{
    constant::{SPOTIFY_EXTERNAL_URL_KEY, SPOTIFY_LIKED_TRACKS_URL},
    model::{playlist::PlaylistType, watcher::Watcher},
};
use askama::Template;
use rspotify::model::{FullPlaylist, Image, SimplifiedPlaylist};
use std::collections::HashMap;

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
    pub all_playlists: Vec<DisplayPlaylist>,
    pub user_playlists: Vec<DisplayPlaylist>,
}

#[derive(Debug, Clone)]
pub struct DisplayPlaylist {
    pub uri: Option<String>,
    pub name: String,
    pub image_url: Option<String>,
    pub spotify_url: String,
}

#[derive(Debug)]
struct PlaylistItem {
    pub kind: PlaylistType,
    pub display: DisplayPlaylist,
}

impl DashboardTemplate {
    fn get_mapped_display_data(
        &self,
        watcher: &Watcher,
    ) -> (Option<PlaylistItem>, Option<PlaylistItem>) {
        (
            self.map_display_data(&watcher.playlist_from),
            self.map_display_data(&watcher.playlist_to),
        )
    }

    fn map_display_data(&self, playlist: &PlaylistType) -> Option<PlaylistItem> {
        match playlist {
            PlaylistType::Saved => Some(PlaylistItem {
                kind: playlist.clone(),
                display: DisplayPlaylist {
                    uri: None,
                    name: playlist.to_string(),
                    image_url: None,
                    spotify_url: SPOTIFY_LIKED_TRACKS_URL.into(),
                },
            }),
            PlaylistType::Uri(id) => self
                .all_playlists
                .iter()
                .find(|data| data.uri.as_ref().is_some_and(|uri| *uri == *id))
                .map(|display| PlaylistItem {
                    kind: playlist.clone(),
                    display: display.clone(),
                }),
        }
    }
}

impl From<FullPlaylist> for DisplayPlaylist {
    fn from(data: FullPlaylist) -> Self {
        Self {
            uri: Some(data.id.to_string()),
            name: data.name.clone(),
            image_url: get_display_image(data.images),
            spotify_url: get_external_url(data.external_urls),
        }
    }
}

impl From<SimplifiedPlaylist> for DisplayPlaylist {
    fn from(data: SimplifiedPlaylist) -> Self {
        Self {
            uri: Some(data.id.to_string()),
            name: data.name.clone(),
            image_url: get_display_image(data.images),
            spotify_url: get_external_url(data.external_urls),
        }
    }
}

fn get_display_image(images: Vec<Image>) -> Option<String> {
    match images.len() {
        1 => images.first().map(|image| image.url.clone()),
        2.. => images
            .iter()
            .filter(|image| image.width.is_some())
            .min_by(|a, b| a.width.cmp(&b.width))
            .map(|image| image.url.clone()),
        _ => None,
    }
}

fn get_external_url(external_urls: HashMap<String, String>) -> String {
    external_urls
        .get(SPOTIFY_EXTERNAL_URL_KEY)
        .expect("should include spotify url")
        .clone()
}
