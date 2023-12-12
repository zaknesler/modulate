use super::error::{ClientError, ClientResult};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Regex to parse a Spotify playlist ID from a playlist URL or Spotify URI
const PLAYLIST_URL_RE: &str =
    r"^(?:https?://open\.spotify\.com/playlist/|spotify:playlist:)?([a-zA-Z0-9]{22})";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlaylistId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub String);

impl PlaylistId {
    /// Format the playlist ID as a Spotify URI
    pub fn uri(&self) -> String {
        format!("spotify:playlist:{}", self.0)
    }

    /// Attempt to parse a playlist ID from a valid Spotify URL or URI
    pub fn try_from_input(input: &str) -> ClientResult<Self> {
        Regex::new(&PLAYLIST_URL_RE)?
            .captures(input)
            .and_then(|captures| Some(Self(captures.get(1)?.as_str().to_string())))
            .ok_or_else(|| ClientError::InvalidId(input.to_owned()))
    }
}

impl TrackId {
    /// Format the track ID as a Spotify URI
    pub fn uri(&self) -> String {
        format!("spotify:track:{}", self.0)
    }
}

impl UserId {
    /// Format the user ID as a Spotify URI
    pub fn uri(&self) -> String {
        format!("spotify:user:{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_valid_playlist_uris_and_urls() {
        let expected = PlaylistId("EX3J5Phq9j7KcpkZJskhRP".to_string());
        let test = |id: &str| assert_eq!(PlaylistId::try_from_input(id).unwrap(), expected);

        test("EX3J5Phq9j7KcpkZJskhRP");
        test("spotify:playlist:EX3J5Phq9j7KcpkZJskhRP");
        test("https://open.spotify.com/playlist/EX3J5Phq9j7KcpkZJskhRP");
        test("https://open.spotify.com/playlist/EX3J5Phq9j7KcpkZJskhRP?some=other&query=params");
    }

    #[test]
    fn it_fails_for_bad_playlist_ids() {
        let test = |id: &str| matches!(PlaylistId::try_from_input(id), Err(_));

        test("some bad id");
        test("EX3J5Phq9j7KcpkZJskhR"); // 21 characters
    }
}
