use std::sync::Arc;

use rspotify::{
    clients::BaseClient, scopes, AuthCodeSpotify, ClientCredsSpotify, Config, Credentials, OAuth,
    Token,
};

use crate::context::AppContext;

#[allow(dead_code)]
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

pub async fn get_token_ensure_refreshed(
    user_id: String,
    token: &Token,
    ctx: Arc<AppContext>,
) -> crate::Result<AuthCodeSpotify> {
    let mut client = create_from_token(token.clone());

    let is_expired = client
        .get_token()
        .lock()
        .await
        .unwrap()
        .as_ref()
        .map(|token| token.is_expired())
        .is_some_and(|val| val);

    if is_expired {
        // Create new client with our credentials and add our current token
        client = create_oauth_client(&ctx.config);
        *client.token.lock().await.unwrap() = Some(token.clone());

        // Request a new token
        let token = client
            .refetch_token()
            .await?
            .ok_or_else(|| anyhow::anyhow!("could not refetch token"))?;

        // Override the token in the client
        *client.get_token().lock().await.unwrap() = Some(token.clone());
        client.write_token_cache().await?;

        // Update the token in the database
        ctx.db
            .get()?
            .prepare("UPDATE users SET token = ? WHERE user_id = ?")?
            .execute(&[&serde_json::to_string(&token)?, &user_id])?;
    }

    // If we requested a new token, the client now has it
    Ok(client)
}
