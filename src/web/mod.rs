use crate::{context::AppContext, CONFIG};
use anyhow::anyhow;
use axum::http::{header, HeaderValue, Method};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

mod middleware;
mod router;
mod view;

pub async fn serve(ctx: AppContext) -> crate::Result<()> {
    tracing::info!(
        "Starting web server on {}:{}",
        CONFIG.web.host,
        CONFIG.web.port
    );

    let cors = CONFIG
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

    axum::Server::bind(&format!("{}:{}", CONFIG.web.host, CONFIG.web.port).parse()?)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(())
}
