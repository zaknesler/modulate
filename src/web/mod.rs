use crate::{context::AppContext, CONFIG};
use anyhow::anyhow;
use axum::http::{header, HeaderValue, Method};
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod middleware;
mod router;
mod session;
mod view;

pub async fn serve(ctx: AppContext) -> crate::Result<()> {
    tracing::info!(
        "Starting web server on {}:{}",
        CONFIG.web.host,
        CONFIG.web.port
    );

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .allow_origin(
            CONFIG
                .web
                .allowed_origins
                .iter()
                .map(|origin| origin.parse::<HeaderValue>())
                .collect::<Result<Vec<_>, _>>()?,
        )
        .allow_credentials(true);

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
