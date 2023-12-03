use crate::{context::AppContext, repo::user::UserRepo, CONFIG};
use rspotify::{
    clients::BaseClient, scopes, AuthCodeSpotify, ClientCredsSpotify, Config, Credentials, OAuth,
    Token,
};

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

pub fn create_oauth_client() -> AuthCodeSpotify {
    let creds = Credentials {
        id: CONFIG.spotify.client_id.clone(),
        secret: Some(CONFIG.spotify.client_secret.clone()),
    };

    let oauth = OAuth {
        scopes: scopes!(
            "user-library-read",
            "user-library-modify",
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        ),
        redirect_uri: CONFIG.spotify.callback_uri.clone(),
        ..Default::default()
    };

    AuthCodeSpotify::with_config(creds, oauth, Config::default())
}

pub fn create_from_token(token: Token) -> AuthCodeSpotify {
    AuthCodeSpotify::from_token(token)
}

pub async fn get_token_ensure_refreshed(
    user_id: &str,
    token: &Token,
    ctx: AppContext,
) -> crate::Result<(AuthCodeSpotify, Token)> {
    let mut token = token.clone();
    let mut client = create_from_token(token.clone());

    if token.is_expired() {
        // Create new client with our credentials and add our current token
        client = create_oauth_client();
        *client.token.lock().await.unwrap() = Some(token.clone());

        // Request a new token
        token = client
            .refetch_token()
            .await?
            .ok_or_else(|| anyhow::anyhow!("could not refetch token"))?;

        // Override the token in the client
        *client.get_token().lock().await.unwrap() = Some(token.clone());
        client.write_token_cache().await?;

        // Update the token in the database
        UserRepo::new(ctx.clone()).upsert_user_token(user_id, &serde_json::to_string(&token)?)?;
    }

    // If we requested a new token, the client now has it
    Ok((client, token))
}
