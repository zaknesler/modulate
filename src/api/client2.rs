use super::model;
use reqwest::header;

const SPOTIFY_API_BASE_URL: &str = "https://api.spotify.com/v1";

pub struct Client<'a> {
    token: &'a str,
}

impl<'a> Client<'a> {
    pub fn from_token(token: &'a str) -> Self {
        Self { token }
    }

    fn new_auth_request(&self) -> crate::Result<reqwest::Client> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", self.token).parse()?,
        );

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|err| err.into())
    }

    pub async fn get_playlist(&self, id: &str) -> crate::Result<model::Playlist> {
        self.new_auth_request()?
            .get(format!("{}/playlists/{}", SPOTIFY_API_BASE_URL, id))
            .query(&[(
                "fields",
                "id,uri,name,images,snapshot_id,external_urls(spotify)",
            )])
            .send()
            .await?
            .json::<model::Playlist>()
            .await
            .map_err(|err| err.into())
    }
}
