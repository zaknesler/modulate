#![allow(clippy::result_large_err)]

use clap::Parser as _;
use error::BaseResult;
use futures::{future::FutureExt, pin_mut, select};
use tracing_subscriber::prelude::*;

mod api;
mod args;
mod config;
mod context;
mod db;
mod error;
mod sync;
mod web;

fn main() -> BaseResult<()> {
    // Ensure config dir exists
    config::init_config_dir()?;

    let args = args::Args::parse();
    let config = config::parse(args.config)?;

    // Initialize Sentry if we have a DSN
    let _sentry = sentry::init((
        config.sentry.dsn.clone(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            send_default_pii: true,
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(
            tracing_subscriber::filter::LevelFilter::from(config.log.level.clone()),
        ))
        .with(sentry::integrations::tracing::layer())
        .init();

    match args.command {
        crate::args::Command::Publish { force } => {
            config::init_config_file(force)?;
        }

        args::Command::Start => {
            // Start thread to run web and sync tasks
            if let Err(err) = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(async { start(config).await })
            {
                tracing::error!("{}", err);
                sentry::capture_error(&err);
            };
        }
    }

    Ok(())
}

async fn start(config: config::ModulateConfig) -> BaseResult<()> {
    let db_path = config::get_config_dir()?.join(&config.database.file);

    let db = db::init(&db_path)?;
    let ctx = context::AppContext { db, config };

    // Run web server and sync tasks concurrently
    let web = web::serve(ctx.clone()).fuse();
    let sync = sync::init(ctx).fuse();

    pin_mut!(web, sync);

    // Wait for either process to finish (i.e. return an error) and exit
    select! {
        result = web => result?,
        result = sync => result?,
    };

    Ok(())
}
