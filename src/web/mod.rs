use crate::error::SpotifyResult;
use anyhow::anyhow;
use axum::http::{header, HeaderValue, Method};
use tower_http::trace::TraceLayer;

mod context;
mod router;

pub async fn serve(config: &crate::config::Config) -> SpotifyResult<()> {
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

    let ctx = context::ApiContext {
        config: config.clone(),
    };

    let app = crate::web::router::router(ctx)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    axum::Server::bind(&format!("{}:{}", &config.web.host, &config.web.port).parse()?)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(())
}
