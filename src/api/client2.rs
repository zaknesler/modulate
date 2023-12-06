use super::{model::User, token::Token};
use crate::CONFIG;
use anyhow::anyhow;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::{header, Url};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

const SPOTIFY_OAUTH2_AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const SPOTIFY_OAUTH2_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const SPOTIFY_OAUTH2_SCOPES: &[&str] = &[
    "user-library-read",
    "user-library-modify",
    "playlist-read-private",
    "playlist-read-collaborative",
    "playlist-modify-public",
    "playlist-modify-private",
];

const SPOTIFY_API_BASE_URL: &str = "https://api.spotify.com/v1";

#[derive(Debug, Clone)]
pub struct Client {
    spotify: BasicClient,
    token: Arc<Mutex<Option<Token>>>,
}

impl Client {
    pub fn new() -> crate::Result<Self> {
        let spotify = BasicClient::new(
            ClientId::new(CONFIG.spotify.client_id.clone()),
            Some(ClientSecret::new(CONFIG.spotify.client_secret.clone())),
            AuthUrl::new(SPOTIFY_OAUTH2_AUTH_URL.to_string())?,
            Some(TokenUrl::new(SPOTIFY_OAUTH2_TOKEN_URL.to_string())?),
        )
        .set_redirect_uri(RedirectUrl::new(CONFIG.spotify.callback_uri.clone())?);

        Ok(Self {
            spotify,
            token: Arc::new(Mutex::new(None)),
        })
    }

    pub fn set_token(&self, token: Token) -> crate::Result<&Self> {
        *self.token.lock().map_err(|_| crate::error::Error::MutexLockError)? = Some(token);

        Ok(self)
    }

    pub fn new_authorize_url(&self) -> (Url, CsrfToken) {
        self.spotify
            .authorize_url(|| CsrfToken::new_random())
            .add_scopes(SPOTIFY_OAUTH2_SCOPES.iter().map(|scope| Scope::new(scope.to_string())))
            .url()
    }

    pub async fn request_token(&self, code: String) -> crate::Result<Token> {
        let res = self
            .spotify
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|err| anyhow!(err))?;

        let scopes = res.scopes().expect("spotify returns this").clone();

        let expires_in =
            chrono::Duration::from_std(res.expires_in().expect("spotify returns this"))?;

        // Set the expiration to 1 minute before the "expires_in" duration returned from Spotify
        let expires_at = chrono::Utc::now() + expires_in - chrono::Duration::minutes(1);

        Ok(Token {
            access_token: res.access_token().secret().to_string(),
            expires_in,
            expires_at,
            refresh_token: res.refresh_token().expect("spotify returns this").secret().to_string(),
            scopes: HashSet::from_iter(scopes.iter().map(|scope| scope.to_string())),
        })
    }

    fn create_request(&self) -> crate::Result<reqwest::Client> {
        let access_token = self
            .token
            .lock()
            .map_err(|_| crate::error::Error::MutexLockError)?
            .as_ref()
            .ok_or_else(|| anyhow!("no token"))?
            .access_token
            .clone();

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", access_token).parse()?,
        );

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|err| err.into())
    }

    pub async fn me(&self) -> crate::Result<User> {
        self.create_request()?
            .get(format!("{}/me", SPOTIFY_API_BASE_URL))
            .send()
            .await?
            .json::<User>()
            .await
            .map_err(|err| err.into())
    }

    // pub async fn get_playlist_partial(&self, id: &str) -> crate::Result<model::Playlist> {
    //     self.new_auth_request()?
    //         .get(format!("{}/playlists/{}", SPOTIFY_API_BASE_URL, id))
    //         .query(&[(
    //             "fields",
    //             "id,uri,name,images,snapshot_id,external_urls(spotify)",
    //         )])
    //         .send()
    //         .await?
    //         .json::<model::Playlist>()
    //         .await
    //         .map_err(|err| err.into())
    // }
}
