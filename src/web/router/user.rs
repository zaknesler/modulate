use crate::{
    context::AppContext,
    web::{middleware::auth, view::UserTemplate},
};
use axum::{middleware, response::IntoResponse, routing::get, Extension, Router};
use futures::TryStreamExt;
use rspotify::{prelude::*, AuthCodeSpotify};
use std::sync::Arc;

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/me", get(get_current_user))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

async fn get_current_user(
    Extension(client): Extension<AuthCodeSpotify>,
) -> crate::Result<impl IntoResponse> {
    let user = client.current_user().await?;
    let playlists = client
        .current_user_playlists()
        .try_collect::<Vec<_>>()
        .await?;

    Ok(UserTemplate {
        name: user.id.to_string().split(':').last().unwrap().to_owned(),
        playlists,
    })
}
