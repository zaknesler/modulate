use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Value that represents the built-in "Liked Tracks" playlist, as it has to be handled differently than regular playlists.
pub const LIKED_PLAYLIST_VALUE: &str = "_liked";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlaylistType {
    /// Saved/liked tracks
    Saved,

    /// Playlist on the current user's account (playlist may be modified)
    CurrentUser(String),

    /// Public playlist by Spotify URL (playlist cannot be modified)
    PublicUrl(String),
}

impl Display for PlaylistType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaylistType::Saved => write!(f, "Liked Tracks"),
            PlaylistType::CurrentUser(value) => write!(f, "{}", value),
            PlaylistType::PublicUrl(value) => write!(f, "{}", value),
        }
    }
}

impl PlaylistType {
    /// Convert to value string for storage.
    /// Use `.to_string()` for displaying.
    pub fn to_value(&self) -> String {
        match self {
            PlaylistType::Saved => LIKED_PLAYLIST_VALUE.to_owned(),
            PlaylistType::CurrentUser(value) => format!("user({})", value),
            PlaylistType::PublicUrl(value) => format!("public({})", value),
        }
    }

    /// Convert from value string.
    pub fn try_from_value(value: &str) -> crate::Result<Self> {
        Ok(match value {
            LIKED_PLAYLIST_VALUE => Self::Saved,
            _ => Regex::new(r#"(user|public)\(([^)]+)\)"#)?
                .captures(value)
                .and_then(|captures| {
                    let kind = captures.get(1)?;
                    let inner = captures.get(2)?;
                    match (kind.as_str(), inner.as_str()) {
                        ("user", inner) => Some(Self::CurrentUser(inner.to_owned())),
                        ("public", inner) => Some(Self::PublicUrl(inner.to_owned())),
                        _ => None,
                    }
                })
                .ok_or(crate::error::Error::InvalidPlaylistType(value.to_owned()))?,
        })
    }
}
