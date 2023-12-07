use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlaylistId(pub String);

impl FromStr for PlaylistId {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Regex::new(&r"(?:https?://open\.spotify\.com/playlist/|spotify:playlist:)([a-zA-Z0-9]+)")?
            .captures(s)
            .and_then(|captures| Some(Self(captures.get(1)?.as_str().to_string())))
            .ok_or_else(|| crate::error::Error::InvalidSpotifyId(s.to_owned()))
    }
}

impl PlaylistId {
    pub fn uri(&self) -> String {
        format!("spotify:playlist:{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_fails_for_bad_ids() {
        assert!(matches!(PlaylistId::from_str("some bad id"), Err(_)));
    }

    #[test]
    fn it_parses_uris_and_urls() {
        let expected = PlaylistId("spotify:playlist:EX3J5Phq9j7KcpkZJskhRP".to_string());

        assert_eq!(
            PlaylistId::from_str("spotify:playlist:EX3J5Phq9j7KcpkZJskhRP").unwrap(),
            expected
        );

        assert_eq!(
            PlaylistId::from_str("https://open.spotify.com/playlist/EX3J5Phq9j7KcpkZJskhRP")
                .unwrap(),
            expected
        );

        assert_eq!(
            PlaylistId::from_str(
                "https://open.spotify.com/playlist/EX3J5Phq9j7KcpkZJskhRP?some=other&query=params"
            )
            .unwrap(),
            expected
        );
    }
}
