use crate::{context::AppContext, repo::watcher::WatcherRepo, web::middleware::auth};
use axum::{
    extract::{Form, State},
    middleware,
    response::{IntoResponse, Redirect},
    routing::post,
    Extension, Router,
};
use rspotify::{prelude::*, AuthCodeSpotify};
use serde::Deserialize;
use validator::Validate;

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/watcher", post(create_watcher))
        .route("/watcher/delete", post(delete_watcher))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

#[derive(Debug, Deserialize, Validate)]
struct CreateWatcherParams {
    #[validate(required)]
    playlist: Option<String>,
}

async fn create_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
    Form(data): Form<CreateWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    data.validate()?;
    let user = client.current_user().await?;

    WatcherRepo::new(ctx.clone())
        .create_watcher(&user.id.to_string(), &data.playlist.expect("validated"))?;

    Ok(Redirect::to("/me"))
}

async fn delete_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
) -> crate::Result<impl IntoResponse> {
    let user = client.current_user().await?;

    WatcherRepo::new(ctx.clone()).delete_watcher(&user.id.to_string())?;

    Ok(Redirect::to("/me"))
}
