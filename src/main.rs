use error::SpotifyResult;

mod config;
mod error;

fn main() -> SpotifyResult<()> {
    let config = crate::config::Config::try_parse()?;
    dbg!(&config);

    Ok(())
}
