use error::SpotifyResult;

mod client;
mod config;
mod error;
mod web;

#[tokio::main]
async fn main() -> SpotifyResult<()> {
    let config = crate::config::Config::try_parse()?;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(config.log_level.clone())
        .init();

    crate::web::serve(&config).await?;

    Ok(())
}
