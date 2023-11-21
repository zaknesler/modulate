use crate::config::Config;
use futures::{future::FutureExt, pin_mut, select};
use lazy_static::lazy_static;

mod config;
mod constant;
mod context;
mod error;
mod model;
mod repo;
mod sync;
mod util;
mod web;

lazy_static! {
    pub static ref CONFIG: Config = Config::try_parse().expect("Failed to parse config");
}

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(CONFIG.log_level.clone())
        .init();

    let db = util::db::init_db(CONFIG.db.file.as_ref())?;
    let ctx = context::AppContext { db };

    // Run watcher and web server concurrently
    let watcher = crate::sync::init(ctx.clone()).fuse();
    let web = crate::web::serve(ctx).fuse();

    pin_mut!(watcher, web);

    // Wait for either process to finish (i.e. return an error) and exit
    select! {
        result = watcher => result,
        result = web => result,
    }
}
