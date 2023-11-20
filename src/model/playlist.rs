use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Value that represents the built-in "Liked Tracks" playlist, as it has to be handled differently than regular playlists.
pub const LIKED_PLAYLIST_VALUE: &str = "_liked";

#[derive(Serialize, Deserialize)]
pub enum PlaylistType {
    Saved,
    WithId(String),
}

impl Display for PlaylistType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaylistType::Saved => write!(f, "Liked Tracks"),
            PlaylistType::WithId(value) => write!(f, "{}", value),
        }
    }
}

impl From<PlaylistType> for String {
    /// Convert to data string for storage.
    /// Use `.to_string()` for displaying.
    fn from(value: PlaylistType) -> Self {
        match value {
            PlaylistType::Saved => LIKED_PLAYLIST_VALUE.to_owned(),
            PlaylistType::WithId(value) => value,
        }
    }
}

impl From<String> for PlaylistType {
    /// Convert from data string.
    fn from(value: String) -> Self {
        match value.as_ref() {
            LIKED_PLAYLIST_VALUE => Self::Saved,
            _ => Self::WithId(value),
        }
    }
}
