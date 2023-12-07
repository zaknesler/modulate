use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct PlaylistId(String);

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
