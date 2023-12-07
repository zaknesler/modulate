use serde::Deserialize;

pub mod client;
pub mod id;
pub mod model;
pub mod pagination;
pub mod token;
pub mod util;

/// Spotify URL for a user's "Liked Tracks" playlist.
pub const SPOTIFY_LIKED_TRACKS_URL: &str = "https://open.spotify.com/collection/tracks";

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SpotifyResponse<T> {
    Ok(T),
    Err(SpotifyErrorWrapper),
}

#[derive(Debug, Deserialize)]
pub struct SpotifyErrorWrapper {
    error: SpotifyErrorResponse,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyErrorResponse {
    status: u16,
    message: String,
}

impl From<SpotifyErrorWrapper> for crate::error::Error {
    fn from(value: SpotifyErrorWrapper) -> Self {
        Self::SpotifyApiError {
            status: value.error.status,
            message: value.error.message,
        }
    }
}
