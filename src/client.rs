use rspotify::{ClientCredsSpotify, Credentials};

pub async fn create_client(config: &crate::config::Config) -> crate::Result<ClientCredsSpotify> {
    let creds = Credentials {
        id: config.spotify.client_id.clone(),
        secret: Some(config.spotify.client_secret.clone()),
    };

    let client = ClientCredsSpotify::new(creds);
    client.request_token().await?;
    Ok(client)
}
