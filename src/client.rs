#![allow(dead_code)]

use rspotify::{scopes, AuthCodeSpotify, ClientCredsSpotify, Config, Credentials, OAuth, Token};

pub async fn create_anonymous_client(
    config: &crate::config::Config,
) -> crate::Result<ClientCredsSpotify> {
    let creds = Credentials {
        id: config.spotify.client_id.clone(),
        secret: Some(config.spotify.client_secret.clone()),
    };

    let client = ClientCredsSpotify::new(creds);
    client.request_token().await?;
    Ok(client)
}

pub fn create_oauth_client(config: &crate::config::Config) -> AuthCodeSpotify {
    let creds = Credentials {
        id: config.spotify.client_id.clone(),
        secret: Some(config.spotify.client_secret.clone()),
    };

    let oauth = OAuth {
        scopes: scopes!(
            "playlist-read-private",
            "playlist-modify-private",
            "user-library-read"
        ),
        redirect_uri: config.spotify.callback_uri.clone(),
        ..Default::default()
    };

    AuthCodeSpotify::with_config(creds, oauth, Config::default())
}

pub fn create_from_token(token: Token) -> AuthCodeSpotify {
    AuthCodeSpotify::from_token(token)
}
