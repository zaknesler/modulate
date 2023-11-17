mod client;
mod config;
mod error;
mod web;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let config = crate::config::Config::try_parse()?;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(config.log_level.clone())
        .init();

    crate::web::serve(&config).await?;

    Ok(())
}
