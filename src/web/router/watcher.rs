use crate::{context::AppContext, web::middleware::auth};
use axum::{
    extract::{Form, State},
    middleware,
    response::{IntoResponse, Redirect},
    routing::post,
    Extension, Router,
};
use rspotify::{prelude::*, AuthCodeSpotify};
use serde::Deserialize;
use std::sync::Arc;

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/watcher", post(create_watcher))
        .route("/watcher/delete", post(delete_watcher))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

#[derive(Debug, Deserialize)]
struct CreateWatcherParams {
    playlist: String,
}

async fn create_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<Arc<AppContext>>,
    Form(data): Form<CreateWatcherParams>,
) -> crate::Result<impl IntoResponse> {
    let user = client.current_user().await?;

    ctx.db
        .get()?
        .prepare("INSERT INTO watchers (user_id, playlist_id) VALUES (?, ?)")?
        .execute(&[&user.id.to_string(), &data.playlist])?;

    Ok(Redirect::to("/me"))
}

async fn delete_watcher(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<Arc<AppContext>>,
) -> crate::Result<impl IntoResponse> {
    let user = client.current_user().await?;

    ctx.db
        .get()?
        .prepare("DELETE FROM watchers WHERE user_id = ?")?
        .execute(&[&user.id.to_string()])?;

    Ok(Redirect::to("/me"))
}
