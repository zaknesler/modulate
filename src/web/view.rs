use crate::{
    api::{
        id::PlaylistId,
        model::{Image, PlaylistPartial},
        SPOTIFY_LIKED_TRACKS_URL,
    },
    config::Config,
    db::model::{playlist::PlaylistType, watcher::Watcher},
};
use askama::Template;

#[derive(Template)]
#[template(path = "connect.html")]
pub struct ConnectTemplate {
    pub url: String,
}

#[derive(Debug, Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub config: Config,
    pub name: String,
    pub watchers: Vec<Watcher>,
    pub all_playlists: Vec<DisplayPlaylist>,
    pub user_playlists: Vec<DisplayPlaylist>,
}

#[derive(Debug, Clone)]
pub struct DisplayPlaylist {
    pub id: Option<PlaylistId>,
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
                    id: None,
                    name: playlist.to_string(),
                    image_url: None,
                    spotify_url: SPOTIFY_LIKED_TRACKS_URL.into(),
                },
            }),
            PlaylistType::Id(id) => self
                .all_playlists
                .iter()
                .find(|data| data.id.as_ref().is_some_and(|uri| *uri == *id))
                .map(|display| PlaylistItem {
                    kind: playlist.clone(),
                    display: display.clone(),
                }),
        }
    }
}

impl From<PlaylistPartial> for DisplayPlaylist {
    fn from(data: PlaylistPartial) -> Self {
        Self {
            id: data.id.parse().ok(),
            name: data.name,
            image_url: get_display_image(data.images),
            spotify_url: data.external_urls.spotify,
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
