use super::{
    error::{ClientError, ClientResult},
    id::{PlaylistId, SnapshotId, TrackId},
    model::{self},
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
use reqwest::{header, StatusCode, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
        let redirect_url = RedirectUrl::new(format!("{}/callback", ctx.config.web.public_url))?;
        let oauth = BasicClient::new(
            ClientId::new(ctx.config.spotify.client_id.clone()),
            Some(ClientSecret::new(ctx.config.spotify.client_secret.clone())),
            AuthUrl::new(SPOTIFY_OAUTH2_AUTH_URL.to_string())?,
            Some(TokenUrl::new(SPOTIFY_OAUTH2_TOKEN_URL.to_string())?),
        )
        .set_redirect_uri(redirect_url);

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

    /// Create a client from an existing token and ensure it's refreshed
    pub async fn from_user_ensure_refreshed(
        ctx: AppContext,
        user: crate::db::model::user::User,
    ) -> ClientResult<(Self, crate::db::model::user::User)> {
        let client = Self::new_with_token(ctx.clone(), user.token.clone())?;

        // If token is still valid, don't do anything
        if !user.token.is_expired() {
            return Ok((client, user));
        }

        let refresh_token =
            user.token.refresh_token.ok_or_else(|| ClientError::MissingRefreshToken)?;

        let mut new_token: Token = client
            .oauth
            .exchange_refresh_token(&RefreshToken::new(refresh_token.clone()))
            .request_async(async_http_client)
            .await
            .map_err(|err| anyhow!(err))?
            .try_into()?;

        // Since the auth flow does not return a refresh token, we must use the old one
        new_token.refresh_token = Some(refresh_token);

        tracing::info!("Refreshed token for user {}", user.user_uri);

        // Update user with new token
        let user = UserRepo::new(ctx).upsert_user_token(&user.user_uri, &new_token)?;

        Ok((client, user))
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
            .connection_verbose(true)
            .build()
            .map_err(|err| err.into())
    }

    /// Fetch the current user
    pub async fn current_user(&self) -> ClientResult<model::User> {
        tracing::debug!("GET /me");

        self.map_response(
            self.create_request()?
                .get(format!("{}/me", SPOTIFY_API_BASE_URL))
                .send()
                .await?,
        )
        .await
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
    pub async fn current_user_saved_tracks_remove_ids(
        &self,
        ids: &Vec<TrackId>,
    ) -> ClientResult<()> {
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

        self.map_response(
            self.create_request()?
                .get(format!("{}/playlists/{}", SPOTIFY_API_BASE_URL, id))
                .query(&[(
                    "fields",
                    "id,name,images,snapshot_id,external_urls(spotify)",
                )])
                .send()
                .await?,
        )
        .await
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
                Some("items(is_local,track(id,type))"),
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
        ids: &Vec<TrackId>,
    ) -> ClientResult<Vec<SnapshotId>> {
        tracing::debug!("POST /playlists/{}/tracks", id);

        let mut snapshot_ids = vec![];

        // Map IDs to URIs
        let uris = ids.iter().map(|id| id.uri()).collect::<Vec<_>>();

        // Endpoint can only be sent a maximum of 100 objects
        for uris in uris.chunks(100) {
            let SnapshotResponse { snapshot_id } = self
                .map_response(
                    self.create_request()?
                        .post(format!("{}/playlists/{}/tracks", SPOTIFY_API_BASE_URL, id))
                        .json(&json!({"uris": &uris}))
                        .send()
                        .await?,
                )
                .await?;

            snapshot_ids.push(snapshot_id);
        }

        Ok(snapshot_ids)
    }

    /// Remove tracks from the specified playlist by ID
    pub async fn playlist_remove_ids(
        &self,
        PlaylistId(id): &PlaylistId,
        ids: &Vec<TrackId>,
    ) -> ClientResult<Vec<SnapshotId>> {
        tracing::debug!("DELETE /playlists/{}/tracks", id);

        let mut snapshot_ids = vec![];

        #[derive(Serialize)]
        struct TrackUri {
            uri: String,
        }

        // Map IDs to track URI objects
        let tracks = ids.iter().map(|id| TrackUri { uri: id.uri() }).collect::<Vec<_>>();

        // Endpoint can only be sent a maximum of 100 objects
        for tracks in tracks.chunks(100) {
            let SnapshotResponse { snapshot_id } = self
                .map_response(
                    self.create_request()?
                        .delete(format!("{}/playlists/{}/tracks", SPOTIFY_API_BASE_URL, id))
                        .json(&json!({"tracks": &tracks}))
                        .send()
                        .await?,
                )
                .await?;

            snapshot_ids.push(snapshot_id);
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
            let mut res = self
                .map_response::<PaginatedResponse<T>>(
                    self.create_request()?.get(url).query(&query).send().await?,
                )
                .await?;

            next = res.next;
            items.append(&mut res.items);

            // Once we've made the first request, clear the query params so they don't get duplicated
            if !query.is_empty() {
                query.clear();
            }
        }

        Ok(items)
    }

    /// Map a Spotify response to a generic type and handle any errors.
    async fn map_response<T>(&self, res: reqwest::Response) -> ClientResult<T>
    where
        T: DeserializeOwned,
    {
        // Spotify doesn't return uniform 429 errors, so handle the status code explicitly
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(ClientError::TooManyRequests);
        }

        let body = res.text().await?;

        // Attempt to map to structured response
        match serde_json::from_str::<SpotifyResponse<T>>(&body)? {
            SpotifyResponse::Success(res) => Ok(res),
            SpotifyResponse::Error(err) => Err(err.into()),
        }
    }
}
