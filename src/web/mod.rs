use anyhow::anyhow;
use axum::http::{header, HeaderValue, Method};
use r2d2_sqlite::rusqlite::params;
use std::{path, sync::Arc};
use tower_http::trace::TraceLayer;

use crate::config::CONFIG_DIR;

mod context;
mod router;
mod view;

pub async fn serve(config: &crate::config::Config) -> crate::Result<()> {
    tracing::info!(
        "Starting web server on {}:{}",
        &config.web.host,
        &config.web.port
    );

    let cors = config
        .web
        .allowed_origins
        .iter()
        .fold(tower_http::cors::CorsLayer::new(), |cors, origin| {
            cors.allow_origin(HeaderValue::from_str(origin).expect("Invalid origin"))
        })
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    let db_path = path::Path::new(CONFIG_DIR).join(&config.db.file);
    let db_manager = r2d2_sqlite::SqliteConnectionManager::file(db_path);
    let db = r2d2::Pool::new(db_manager)?;

    // Ensure db table exists
    db.get()?.execute(
        "CREATE TABLE IF NOT EXISTS tokens (token VARCHAR)",
        params![],
    )?;

    let ctx = Arc::new(context::ApiContext {
        config: config.clone(),
        db,
    });

    let app = crate::web::router::router(ctx)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    axum::Server::bind(&format!("{}:{}", &config.web.host, &config.web.port).parse()?)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(())
}
