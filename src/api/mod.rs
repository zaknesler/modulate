use serde::Deserialize;

use self::error::ClientError;

pub mod client;
pub mod error;
pub mod id;
pub mod model;
pub mod pagination;
pub mod token;
pub mod util;

/// Spotify URL for a user's "Liked Tracks" playlist.
pub const SPOTIFY_LIKED_TRACKS_URL: &str = "https://open.spotify.com/collection/tracks";

#[derive(Debug, Deserialize)]
pub struct SnapshotResponse {
    snapshot_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SpotifyResponse<T> {
    Success(T),
    Error(SpotifyErrorWrapper),
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

impl From<SpotifyErrorWrapper> for ClientError {
    fn from(value: SpotifyErrorWrapper) -> Self {
        Self::ApiError {
            status: value.error.status,
            message: value.error.message,
        }
    }
}
