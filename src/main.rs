use core::panic;

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

fn main() {
    dotenvy::dotenv().expect("missing .env file");
    let config = Config::try_parse().expect("could not parse config");

    // Initialize Sentry if we have a DSN
    let _sentry = sentry::init((
        config.sentry.dsn.clone(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(config.log.level.clone()).init();

    // Start thread to run web and sync tasks
    if let Err(err) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run(config).await })
    {
        tracing::error!("{}", err);
        sentry::capture_error(&err);
    }
}

async fn run(config: Config) -> BaseResult<()> {
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
