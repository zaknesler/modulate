use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
            PlaylistType::Saved => write!(f, "{}", LIKED_PLAYLIST_VALUE),
            PlaylistType::WithId(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<String> for PlaylistType {
    type Error = crate::error::Error;

    fn try_from(value: String) -> crate::Result<Self> {
        Ok(match value.as_ref() {
            LIKED_PLAYLIST_VALUE => Self::Saved,
            _ => Self::WithId(value),
        })
    }
}
