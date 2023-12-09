use self::error::WebResult;
use crate::context::AppContext;
use axum::http::{header, HeaderValue, Method};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod error;
mod middleware;
mod response;
mod router;
mod session;
mod util;
mod view;

pub async fn serve(ctx: AppContext) -> WebResult<()> {
    tracing::info!(
        "Starting web server on {}:{}",
        ctx.config.web.host,
        ctx.config.web.port
    );

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .allow_origin(
            ctx.config
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

    axum::serve(
        TcpListener::bind(format!("{}:{}", ctx.config.web.host, ctx.config.web.port)).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}
