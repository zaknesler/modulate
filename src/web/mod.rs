use crate::context::AppContext;
use anyhow::anyhow;
use axum::http::{header, HeaderValue, Method};
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

mod middleware;
mod router;
mod view;

pub async fn serve(ctx: Arc<AppContext>) -> crate::Result<()> {
    tracing::info!(
        "Starting web server on {}:{}",
        &ctx.config.web.host,
        &ctx.config.web.port
    );

    let cors = ctx
        .config
        .web
        .allowed_origins
        .iter()
        .fold(tower_http::cors::CorsLayer::new(), |cors, origin| {
            cors.allow_origin(HeaderValue::from_str(origin).expect("Invalid origin"))
        })
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    let app = crate::web::router::router(ctx.clone())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&format!("{}:{}", &ctx.config.web.host, &ctx.config.web.port).parse()?)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(())
}
