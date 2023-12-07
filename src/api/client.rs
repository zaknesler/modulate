use super::{
    id::PlaylistId,
    model::{self, User},
    pagination::PaginatedResponse,
    token::Token,
    SpotifyResponse,
};
use crate::{
    api::model::{TrackPartial, TrackType},
    context::AppContext,
    repo::user::UserRepo,
    CONFIG,
};
use anyhow::anyhow;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, RefreshToken, Scope, TokenUrl,
};
use reqwest::{header, Url};
use serde::Deserialize;
use serde_json::json;
use std::sync::{Arc, Mutex};

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
        user_uri: &str,
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

        if token.refresh_token.is_none() {
            return Err(anyhow!("missing refresh token").into());
        }

        let mut new_token: Token = self
            .oauth
            .exchange_refresh_token(&RefreshToken::new(token.refresh_token.clone().unwrap()))
            .request_async(async_http_client)
            .await
            .map_err(|err| anyhow!(err))?
            .try_into()?;

        // Since the auth flow does not return a refresh token, we must use the old one
        new_token.refresh_token = Some(token.refresh_token.unwrap());

        self.set_token(new_token.clone())?;

        // Update user
        UserRepo::new(ctx).upsert_user_token(user_uri, &new_token)?;

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

    pub async fn current_user(&self) -> crate::Result<model::User> {
        self.create_request()?
            .get(format!("{}/me", SPOTIFY_API_BASE_URL))
            .send()
            .await?
            .json::<User>()
            .await
            .map_err(|err| err.into())
    }

    pub async fn current_user_playlists(&self) -> crate::Result<Vec<model::PlaylistPartial>> {
        self.collect_paginated(
            format!("{}/me/playlists", SPOTIFY_API_BASE_URL).as_ref(),
            None,
        )
        .await
        .map_err(|err| err.into())
    }

    pub async fn current_user_saved_track_partials(
        &self,
    ) -> crate::Result<Vec<model::PlaylistPartial>> {
        self.collect_paginated(
            format!("{}/me/playlists", SPOTIFY_API_BASE_URL).as_ref(),
            None,
        )
        .await
        .map_err(|err| err.into())
    }

    pub async fn current_user_saved_tracks_remove_ids(&self, ids: &[&str]) -> crate::Result<()> {
        // Endpoint can only be sent a maximum of 50 IDs
        for ids in ids.chunks(50) {
            self.create_request()?
                .delete(format!("{}/me/tracks", SPOTIFY_API_BASE_URL))
                .json(&json!({"ids": &ids.join(",")}))
                .send()
                .await?
                .json::<String>()
                .await?;
        }

        Ok(())
    }

    pub async fn playlist(
        &self,
        PlaylistId(id): &PlaylistId,
    ) -> crate::Result<model::PlaylistPartial> {
        let res = self
            .create_request()?
            .get(format!("{}/playlists/{}", SPOTIFY_API_BASE_URL, id))
            .query(&[(
                "fields",
                "id,uri,name,images,snapshot_id,external_urls(spotify)",
            )])
            .send()
            .await?
            .json::<SpotifyResponse<model::PlaylistPartial>>()
            .await?;

        Ok(match res {
            SpotifyResponse::Ok(res) => res,
            SpotifyResponse::Err(err) => return Err(err.into()),
        })
    }

    /// Get a list of all track IDs in a playlist
    pub async fn playlist_track_partials(
        &self,
        PlaylistId(id): &PlaylistId,
    ) -> crate::Result<Vec<TrackPartial>> {
        #[derive(Deserialize)]
        struct TrackPartialWrapper {
            is_local: bool,
            track: TrackPartial,
        }

        Ok(self
            .collect_paginated::<TrackPartialWrapper>(
                format!("{}/playlists/{}/tracks", SPOTIFY_API_BASE_URL, id).as_ref(),
                Some("items(is_local,track(id,uri,type))"),
            )
            .await?
            .into_iter()
            .filter_map(|item| {
                (!item.is_local && matches!(item.track.kind, TrackType::Track)).then(|| item.track)
            })
            .collect::<Vec<_>>())
    }

    pub async fn playlist_add_uris(
        &self,
        PlaylistId(id): &PlaylistId,
        uris: &[&str],
    ) -> crate::Result<model::PlaylistPartial> {
        self.create_request()?
            .post(format!("{}/playlists/{}", SPOTIFY_API_BASE_URL, id))
            .json(&json!({"uris": &uris.join(",")}))
            .send()
            .await?
            .json::<model::PlaylistPartial>()
            .await
            .map_err(|err| err.into())
    }

    pub async fn playlist_remove_uris(
        &self,
        PlaylistId(id): &PlaylistId,
        uris: &[&str],
    ) -> crate::Result<Vec<String>> {
        let mut snapshot_ids = vec![];

        // Endpoint can only be sent a maximum of 100 objects
        for uris in uris.chunks(100) {
            snapshot_ids.push(
                self.create_request()?
                    .delete(format!("{}/playlists/{}", SPOTIFY_API_BASE_URL, id))
                    .json(&json!({"uris": &uris.join(",")}))
                    .send()
                    .await?
                    .json::<String>()
                    .await?,
            );
        }

        Ok(snapshot_ids)
    }

    /// Make the GET requests needed to paginate through all records given a URL
    async fn collect_paginated<T>(&self, url: &str, fields: Option<&str>) -> crate::Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut items = vec![];
        let mut next = Some(url.to_string());

        let mut query = vec![("limit", "50".to_string())];

        if let Some(fields) = fields {
            query.push((
                "fields",
                ["next,previous,limit,offset,total", fields].join(","),
            ));
        }

        // TODO: make requests concurrent
        while let Some(url) = next {
            let mut res = self
                .create_request()?
                .get(url)
                .query(&query)
                .send()
                .await?
                .json::<PaginatedResponse<T>>()
                .await?;

            next = res.next;
            items.append(&mut res.items);
        }

        Ok(items)
    }
}
