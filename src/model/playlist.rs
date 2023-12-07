use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Value that represents the built-in "Liked Tracks" playlist, as it has to be handled differently than regular playlists.
pub const LIKED_PLAYLIST_VALUE: &str = "_liked";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlaylistType {
    Saved,
    Uri(String),
}

impl Display for PlaylistType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaylistType::Saved => write!(f, "Liked Tracks"),
            PlaylistType::Uri(value) => write!(f, "{}", value),
        }
    }
}

impl PlaylistType {
    /// Convert to value string for storage.
    /// Use `.to_string()` for displaying.
    pub fn to_value(&self) -> &str {
        match self {
            PlaylistType::Saved => LIKED_PLAYLIST_VALUE,
            PlaylistType::Uri(id) => id,
        }
    }

    /// Convert from value string.
    pub fn try_from_value(value: &str) -> crate::Result<Self> {
        match value {
            LIKED_PLAYLIST_VALUE => Ok(Self::Saved),
            _ => try_extract_playlist_uri(value).map(|uri| Self::Uri(uri)),
        }
    }
}

fn try_extract_playlist_uri(value: &str) -> crate::Result<String> {
    Regex::new(r"(?:https?://open\.spotify\.com/playlist/|spotify:playlist:)([a-zA-Z0-9]+)")?
        .captures(value)
        .and_then(|captures| Some(format!("spotify:playlist:{}", captures.get(1)?.as_str())))
        .ok_or_else(|| crate::error::Error::InvalidSpotifyId(value.to_owned()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_fails_for_bad_ids() {
        assert!(matches!(try_extract_playlist_uri("some bad id"), Err(_)));
    }

    #[test]
    fn it_parses_uris_and_urls() {
        let expected = "spotify:playlist:EX3J5Phq9j7KcpkZJskhRP".to_string();

        assert_eq!(
            try_extract_playlist_uri("spotify:playlist:EX3J5Phq9j7KcpkZJskhRP").unwrap(),
            expected
        );

        assert_eq!(
            try_extract_playlist_uri("https://open.spotify.com/playlist/EX3J5Phq9j7KcpkZJskhRP")
                .unwrap(),
            expected
        );

        assert_eq!(
            try_extract_playlist_uri(
                "https://open.spotify.com/playlist/EX3J5Phq9j7KcpkZJskhRP?some=other&query=params"
            )
            .unwrap(),
            expected
        );
    }
}
