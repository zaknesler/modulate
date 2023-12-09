use crate::config::Config;
use error::BaseResult;
use futures::{future::FutureExt, pin_mut, select};

mod api;
mod config;
mod context;
mod db;
mod error;
mod sync;
mod web;

#[tokio::main]
async fn main() -> BaseResult<()> {
    dotenvy::dotenv()?;
    let config = Config::try_parse()?;

    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(config.log.level.clone()).init();

    let db = db::init(config.database.url.as_ref())?;
    let ctx = context::AppContext { db, config };

    // Run sync task and web server concurrently
    let sync = crate::sync::init(ctx.clone()).fuse();
    let web = crate::web::serve(ctx).fuse();

    pin_mut!(sync, web);

    // Wait for either process to finish (i.e. return an error) and exit
    Ok(select! {
        result = sync => result?,
        result = web => result?,
    })
}
