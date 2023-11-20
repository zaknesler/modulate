use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Value that represents the built-in "Liked Tracks" playlist, as it has to be handled differently than regular playlists.
pub const LIKED_PLAYLIST_VALUE: &str = "_liked";

#[derive(Debug, Serialize, Deserialize)]
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

impl PlaylistType {
    /// Convert to value string for storage.
    /// Use `.to_string()` for displaying.
    pub fn to_value(&self) -> &str {
        match self {
            PlaylistType::Saved => LIKED_PLAYLIST_VALUE,
            PlaylistType::WithId(value) => value,
        }
    }

    /// Convert from value string.
    pub fn from_value(value: &str) -> Self {
        match value {
            LIKED_PLAYLIST_VALUE => Self::Saved,
            _ => Self::WithId(value.to_owned()),
        }
    }
}
