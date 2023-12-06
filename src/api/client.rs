use super::{
    model::{self, User},
    pagination::PaginatedResponse,
    token::Token,
};
use crate::{context::AppContext, repo::user::UserRepo, CONFIG};
use anyhow::anyhow;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, RefreshToken, Scope, TokenUrl,
};
use reqwest::{header, Url};
use serde::Deserialize;
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
    oauth: BasicClient,
    token: Arc<Mutex<Option<Token>>>,
}

impl Client {
    pub fn new() -> crate::Result<Self> {
        let oauth = BasicClient::new(
            ClientId::new(CONFIG.spotify.client_id.clone()),
            Some(ClientSecret::new(CONFIG.spotify.client_secret.clone())),
            AuthUrl::new(SPOTIFY_OAUTH2_AUTH_URL.to_string())?,
            Some(TokenUrl::new(SPOTIFY_OAUTH2_TOKEN_URL.to_string())?),
        )
        .set_redirect_uri(RedirectUrl::new(CONFIG.spotify.callback_uri.clone())?);

        Ok(Self {
            oauth,
            token: Arc::new(Mutex::new(None)),
        })
    }

    pub fn new_with_token(token: Token) -> crate::Result<Self> {
        let client = Self::new()?;
        client.set_token(token)?;
        Ok(client)
    }

    pub fn set_token(&self, token: Token) -> crate::Result<&Self> {
        *self.token.lock().map_err(|_| crate::error::Error::MutexLockError)? = Some(token);
        Ok(self)
    }

    pub fn new_authorize_url(&self) -> (Url, CsrfToken) {
        self.oauth
            .authorize_url(|| CsrfToken::new_random())
            .add_scopes(SPOTIFY_OAUTH2_SCOPES.iter().map(|scope| Scope::new(scope.to_string())))
            .url()
    }

    pub async fn get_token_from_code(&self, code: String) -> crate::Result<Token> {
        self.oauth
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|err| anyhow!(err))?
            .try_into()
    }

    /// Fetch a new access token if the current one is expired, and update the user's token in the DB
    pub async fn ensure_token_refreshed(
        &self,
        ctx: AppContext,
        user_id: &str,
    ) -> crate::Result<&Self> {
        let token = self
            .token
            .lock()
            .map_err(|_| crate::error::Error::MutexLockError)?
            .as_ref()
            .ok_or_else(|| anyhow!("no token"))?
            .clone();

        // If token is still valid, don't do anything
        if !token.is_expired() {
            return Ok(self);
        }

        let token: Token = self
            .oauth
            .exchange_refresh_token(&RefreshToken::new(token.refresh_token))
            .request_async(async_http_client)
            .await
            .map_err(|err| anyhow!(err))?
            .try_into()?;

        self.set_token(token.clone())?;

        // Update user
        UserRepo::new(ctx).upsert_user_token(user_id, &token)?;

        Ok(self)
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

    pub async fn get_current_user(&self) -> crate::Result<model::User> {
        self.create_request()?
            .get(format!("{}/me", SPOTIFY_API_BASE_URL))
            .send()
            .await?
            .json::<User>()
            .await
            .map_err(|err| err.into())
    }

    pub async fn get_current_user_playlists(&self) -> crate::Result<Vec<model::Playlist>> {
        // TODO: paginate
        Ok(self
            .create_request()?
            .get(format!("{}/me/playlists", SPOTIFY_API_BASE_URL))
            .send()
            .await?
            .json::<PaginatedResponse<model::Playlist>>()
            .await?
            .items)
    }

    pub async fn get_playlist(&self, id: &str) -> crate::Result<model::Playlist> {
        self.create_request()?
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

    /// Get a list of all track IDs in a playlist
    pub async fn get_playlist_track_ids(&self, id: &str) -> crate::Result<HashSet<String>> {
        #[derive(Deserialize)]
        struct TrackPartialWrapper {
            is_local: bool,
            track: TrackPartial,
        }

        #[derive(Deserialize)]
        struct TrackPartial {
            id: String,
            #[serde(rename = "type")]
            kind: String,
        }

        // TODO: paginate

        Ok(self
            .create_request()?
            .get(format!("{}/playlists/{}/tracks", SPOTIFY_API_BASE_URL, id))
            .query(&[("fields", "items(is_local,track(id,type))")])
            .send()
            .await?
            .json::<PaginatedResponse<TrackPartialWrapper>>()
            .await?
            .items
            .into_iter()
            .filter_map(|item| (item.is_local && item.track.kind == "track").then(|| item.track.id))
            .collect::<HashSet<_>>())
    }
}
