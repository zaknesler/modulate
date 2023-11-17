use error::SpotifyResult;
use rspotify::{model::UserId, prelude::*};

mod client;
mod config;
mod error;

#[tokio::main]
async fn main() -> SpotifyResult<()> {
    let config = crate::config::Config::try_parse()?;
    let client = crate::client::create_client(&config).await?;

    let user = client.user(UserId::from_id("zaknes")?).await?;
    dbg!(&user);

    Ok(())
}
