use super::error::ClientError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlaylistId(pub String);

impl FromStr for PlaylistId {
    type Err = ClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Regex::new(
            &r"^(?:https?://open\.spotify\.com/playlist/|spotify:playlist:)?([a-zA-Z0-9]{22})",
        )?
        .captures(s)
        .and_then(|captures| Some(Self(captures.get(1)?.as_str().to_string())))
        .ok_or_else(|| ClientError::InvalidId(s.to_owned()))
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
        let test = |id: &str| matches!(PlaylistId::from_str(id), Err(_));

        test("some bad id");
        test("EX3J5Phq9j7KcpkZJskhR"); // 21 characters
    }

    #[test]
    fn it_parses_uris_and_urls() {
        let expected = PlaylistId("EX3J5Phq9j7KcpkZJskhRP".to_string());
        let test = |id: &str| assert_eq!(PlaylistId::from_str(id).unwrap(), expected);

        test("EX3J5Phq9j7KcpkZJskhRP");
        test("spotify:playlist:EX3J5Phq9j7KcpkZJskhRP");
        test("https://open.spotify.com/playlist/EX3J5Phq9j7KcpkZJskhRP");
        test("https://open.spotify.com/playlist/EX3J5Phq9j7KcpkZJskhRP?some=other&query=params");
    }
}
