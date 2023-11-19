use std::sync::Arc;

use futures::future;

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
    let ctx = Arc::new(context::AppContext { config, db });

    // Run watcher and web server concurrently
    let (_, res) = future::join(crate::watcher::init(ctx.clone()), crate::web::serve(ctx)).await;
    res
}
