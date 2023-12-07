use crate::api::id::PlaylistId;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Value that represents the built-in "Liked Tracks" playlist, as it has to be handled differently than regular playlists.
pub const LIKED_PLAYLIST_VALUE: &str = "_liked";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlaylistType {
    Saved,
    Id(PlaylistId),
}

impl Display for PlaylistType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaylistType::Saved => write!(f, "Liked Tracks"),
            PlaylistType::Id(PlaylistId(value)) => write!(f, "{}", value),
        }
    }
}

impl PlaylistType {
    /// Convert to value string for storage.
    /// Use `.to_string()` for displaying.
    pub fn to_value(&self) -> String {
        match self {
            PlaylistType::Saved => LIKED_PLAYLIST_VALUE.to_string(),
            PlaylistType::Id(id) => id.uri(),
        }
    }

    /// Convert from value string.
    pub fn try_from_value(value: &str) -> crate::Result<Self> {
        Ok(match value {
            LIKED_PLAYLIST_VALUE => Self::Saved,
            _ => Self::Id(value.parse()?),
        })
    }
}
