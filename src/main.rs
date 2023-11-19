use futures::{future::FutureExt, pin_mut, select};

mod config;
mod context;
mod error;
mod util;
mod watcher;
mod web;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let config = crate::config::Config::try_parse()?;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(config.log_level.clone())
        .init();

    let db = util::db::init_db(config.db.file.clone())?;
    let ctx = context::AppContext { config, db };

    // Run watcher and web server concurrently
    let watcher = crate::watcher::init(ctx.clone()).fuse();
    let web = crate::web::serve(ctx).fuse();

    pin_mut!(watcher, web);

    // Wait for either process to finish (i.e. return an error) and exit
    select! {
        result = watcher => result,
        result = web => result,
    }
}
