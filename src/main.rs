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
    let config = Config::try_parse()?;

    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(config.log_level.clone()).init();

    let db = db::init(config.db.file.as_ref())?;
    let ctx = context::AppContext { db, config };

    // Run watcher and web server concurrently
    let watcher = crate::sync::init(ctx.clone()).fuse();
    let web = crate::web::serve(ctx).fuse();

    pin_mut!(watcher, web);

    // Wait for either process to finish (i.e. return an error) and exit
    select! {
        result = watcher => result.map_err(|err| err.into()),
        result = web => result.map_err(|err| err.into()),
    }
}
