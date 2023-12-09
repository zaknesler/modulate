use super::{
    error::{ClientError, ClientResult},
    id::{PlaylistId, SnapshotId},
    model::{self, User},
    response::PaginatedResponse,
    token::Token,
};
use crate::{
    api::{
        model::TrackPartial,
        response::{SnapshotResponse, SpotifyResponse},
    },
    context::AppContext,
    db::repo::user::UserRepo,
};
use anyhow::anyhow;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, RefreshToken, Scope, TokenUrl,
};
use reqwest::{header, Url};
use serde::Deserialize;
use serde_json::json;
use std::{
    fmt::Debug,
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Client {
    ctx: AppContext,
    oauth: BasicClient,
    token: Arc<Mutex<Option<Token>>>,
}

impl Client {
    /// Initialize a client with our Spotify credentials
    pub fn new(ctx: AppContext) -> ClientResult<Self> {
        let oauth = BasicClient::new(
            ClientId::new(ctx.config.spotify.client_id.clone()),
            Some(ClientSecret::new(ctx.config.spotify.client_secret.clone())),
            AuthUrl::new(SPOTIFY_OAUTH2_AUTH_URL.to_string())?,
            Some(TokenUrl::new(SPOTIFY_OAUTH2_TOKEN_URL.to_string())?),
        )
        .set_redirect_uri(RedirectUrl::new(ctx.config.spotify.callback_uri.clone())?);

        Ok(Self {
            ctx,
            oauth,
            token: Arc::new(Mutex::new(None)),
        })
    }

    /// Create a client from an existing token
    pub fn new_with_token(ctx: AppContext, token: Token) -> ClientResult<Self> {
        let client = Self::new(ctx)?;
        client.set_token(token)?;
        Ok(client)
    }

    /// Set the token within the client to be used for subsequent requests
    pub fn set_token(&self, token: Token) -> ClientResult<&Self> {
        *self.token.lock().map_err(|_| ClientError::MutexLockError)? = Some(token);
        Ok(self)
    }

    /// Generate a new URL to authorize a user, along with a CSRF token to be verified from Spotify's response
    pub fn new_authorize_url(&self) -> (Url, CsrfToken) {
        self.oauth
            .authorize_url(|| CsrfToken::new_random())
            .add_scopes(SPOTIFY_OAUTH2_SCOPES.iter().map(|scope| Scope::new(scope.to_string())))
            .url()
    }

    /// Using the code returned from Spotify during the OAuth2 process, fetch the token data
    pub async fn get_token_from_code(&self, code: String) -> ClientResult<Token> {
        self.oauth
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|err| anyhow!(err))?
            .try_into()
    }

    /// Fetch a new access token if the current one is expired, and update the user's token in the DB
    pub async fn ensure_token_refreshed(&self, user_uri: &str) -> ClientResult<&Self> {
        let token = self
            .token
            .lock()
            .map_err(|_| ClientError::MutexLockError)?
            .as_ref()
            .ok_or_else(|| ClientError::MissingToken)?
            .clone();

        // If token is still valid, don't do anything
        if !token.is_expired() {
            return Ok(self);
        }

        let refresh_token = token.refresh_token.ok_or_else(|| ClientError::MissingRefreshToken)?;

        let mut new_token: Token = self
            .oauth
            .exchange_refresh_token(&RefreshToken::new(refresh_token.clone()))
            .request_async(async_http_client)
            .await
            .map_err(|err| anyhow!(err))?
            .try_into()?;

        // Since the auth flow does not return a refresh token, we must use the old one
        new_token.refresh_token = Some(refresh_token);

        self.set_token(new_token.clone())?;

        tracing::info!("Refreshed token for user {}", user_uri);

        // Update user
        UserRepo::new(self.ctx.clone()).upsert_user_token(user_uri, &new_token)?;

        Ok(self)
    }

    /// Create a request client with the appropriate authorization headers
    fn create_request(&self) -> ClientResult<reqwest::Client> {
        let access_token = self
            .token
            .lock()
            .map_err(|_| ClientError::MutexLockError)?
            .as_ref()
            .ok_or_else(|| ClientError::MissingAccessToken)?
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

    /// Fetch the current user
    pub async fn current_user(&self) -> ClientResult<model::User> {
        tracing::debug!("GET /me");

        let res = self
            .create_request()?
            .get(format!("{}/me", SPOTIFY_API_BASE_URL))
            .send()
            .await?
            .json::<SpotifyResponse<User>>()
            .await?;

        Ok(match res {
            SpotifyResponse::Success(res) => res,
            SpotifyResponse::Error(err) => return Err(err.into()),
        })
    }

    /// Get all playlists saved by the current user, returning only basic display data
    pub async fn current_user_playlists(&self) -> ClientResult<Vec<model::PlaylistPartial>> {
        tracing::debug!("GET /me/playlists");

        self.collect_paginated(
            format!("{}/me/playlists", SPOTIFY_API_BASE_URL).as_ref(),
            None,
        )
        .await
        .map_err(|err| err.into())
    }

    /// Get all tracks saved by the current user, returning only the ID/URI data
    pub async fn current_user_saved_track_partials(
        &self,
    ) -> ClientResult<Vec<model::TrackPartial>> {
        tracing::debug!("GET /me/tracks");

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            track: model::TrackPartial,
        }

        Ok(self
            .collect_paginated::<Wrapper>(
                format!("{}/me/tracks", SPOTIFY_API_BASE_URL).as_ref(),
                None,
            )
            .await?
            .into_iter()
            .map(|wrapper| wrapper.track)
            .collect::<Vec<_>>())
    }

    /// Remove tracks from the current user's saved tracks by ID
    pub async fn current_user_saved_tracks_remove_ids(&self, ids: &[&str]) -> ClientResult<()> {
        tracing::debug!("DELETE /me/tracks");

        // Endpoint can only be sent a maximum of 50 IDs
        for ids in ids.chunks(50) {
            let res = self
                .create_request()?
                .delete(format!("{}/me/tracks", SPOTIFY_API_BASE_URL))
                .json(&json!({ "ids": ids }))
                .send()
                .await?;

            // This endpoint returns nothing when successful, because of course it does
            if res.status().is_success() {
                return Ok(());
            }

            // Manually convert body to JSON if unsuccessful to get status and error
            match serde_json::from_str::<SpotifyResponse<()>>(&res.text().await?)? {
                SpotifyResponse::Error(err) => return Err(err.into()),
                _ => {}
            }
        }

        Ok(())
    }

    /// Get all a playlist by ID, returning only basic display data
    pub async fn playlist_partial(
        &self,
        PlaylistId(id): &PlaylistId,
    ) -> ClientResult<model::PlaylistPartial> {
        tracing::debug!("GET /playlists/{}", id);

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
            SpotifyResponse::Success(res) => res,
            SpotifyResponse::Error(err) => return Err(err.into()),
        })
    }

    /// Get all tracks in a playlist, returning only the ID/URI data
    pub async fn playlist_track_partials(
        &self,
        PlaylistId(id): &PlaylistId,
    ) -> ClientResult<Vec<TrackPartial>> {
        tracing::debug!("GET /playlists/{}/tracks", id);

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            is_local: bool,
            track: Option<TrackPartial>,
        }

        Ok(self
            .collect_paginated::<Wrapper>(
                format!("{}/playlists/{}/tracks", SPOTIFY_API_BASE_URL, id).as_ref(),
                Some("items(is_local,track(id,uri,type))"),
            )
            .await?
            .into_iter()
            .filter_map(|item| match (item.is_local, item.track) {
                (false, Some(track)) => Some(track),
                _ => None,
            })
            .collect::<Vec<_>>())
    }

    /// Add tracks to the specified playlist by ID
    pub async fn playlist_add_ids(
        &self,
        PlaylistId(id): &PlaylistId,
        ids: &[&str],
    ) -> ClientResult<Vec<SnapshotId>> {
        tracing::debug!("POST /playlists/{}/tracks", id);

        let mut snapshot_ids = vec![];

        // Map IDs to URIs
        let uris = ids.iter().map(|id| format!("spotify:track:{}", id)).collect::<Vec<_>>();

        // Endpoint can only be sent a maximum of 100 objects
        for uris in uris.chunks(100) {
            let res = self
                .create_request()?
                .post(format!("{}/playlists/{}/tracks", SPOTIFY_API_BASE_URL, id))
                .json(&json!({"uris": &uris}))
                .send()
                .await?
                .json::<SpotifyResponse<SnapshotResponse>>()
                .await?;

            match res {
                SpotifyResponse::Success(SnapshotResponse { snapshot_id }) => {
                    snapshot_ids.push(snapshot_id)
                }
                SpotifyResponse::Error(err) => return Err(err.into()),
            };
        }

        Ok(snapshot_ids)
    }

    /// Remove tracks from the specified playlist by ID
    pub async fn playlist_remove_ids(
        &self,
        PlaylistId(id): &PlaylistId,
        ids: &[&str],
    ) -> ClientResult<Vec<SnapshotId>> {
        tracing::debug!("DELETE /playlists/{}/tracks", id);

        let mut snapshot_ids = vec![];

        // Map IDs to URIs
        let uris = ids.iter().map(|id| format!("spotify:track:{}", id)).collect::<Vec<_>>();

        // Endpoint can only be sent a maximum of 100 objects
        for uris in uris.chunks(100) {
            let res = self
                .create_request()?
                .delete(format!("{}/playlists/{}/tracks", SPOTIFY_API_BASE_URL, id))
                .json(&json!({"uris": &uris.join(",")}))
                .send()
                .await?
                .json::<SpotifyResponse<SnapshotResponse>>()
                .await?;

            match res {
                SpotifyResponse::Success(SnapshotResponse { snapshot_id }) => {
                    snapshot_ids.push(snapshot_id)
                }
                SpotifyResponse::Error(err) => return Err(err.into()),
            };
        }

        Ok(snapshot_ids)
    }

    /// Make the GET requests needed to paginate through all records given a URL
    async fn collect_paginated<T>(&self, url: &str, fields: Option<&str>) -> ClientResult<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Debug,
    {
        let mut items = vec![];
        let mut next = Some(url.to_string());
        let mut query = vec![("limit", "50".to_string())];

        // If we're scoping by fields, we want to make sure Spotify returns the pagination fields as well
        if let Some(fields) = fields {
            query.push((
                "fields",
                ["href,next,previous,limit,offset,total", fields].join(","),
            ));
        }

        while let Some(url) = next {
            let res = self
                .create_request()?
                .get(url)
                .query(&query)
                .send()
                .await?
                .json::<SpotifyResponse<PaginatedResponse<T>>>()
                .await?;

            // Once we've made the first request, clear the query params so they don't get duplicated
            if !query.is_empty() {
                query.clear();
            }

            match res {
                SpotifyResponse::Success(mut res) => {
                    next = res.next;
                    items.append(&mut res.items);
                }
                SpotifyResponse::Error(err) => return Err(err.into()),
            };
        }

        Ok(items)
    }
}
