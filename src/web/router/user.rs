use crate::{
    context::AppContext,
    repo::watcher::WatcherRepo,
    web::{middleware::auth, view::DashboardTemplate},
};
use axum::{extract::State, middleware, response::IntoResponse, routing::get, Extension, Router};
use futures::TryStreamExt;
use rspotify::{prelude::*, AuthCodeSpotify};

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/me", get(get_dashboard))
        .route_layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::middleware,
        ))
        .with_state(ctx)
}

async fn get_dashboard(
    Extension(client): Extension<AuthCodeSpotify>,
    State(ctx): State<AppContext>,
) -> crate::Result<impl IntoResponse> {
    let user = client.current_user().await?;

    // Fetch the details about the watched playlist if one exists
    let watchers = WatcherRepo::new(ctx.clone()).get_all_watchers_by_user(&user.id.to_string())?;

    // If the user doesn't have a watcher configured, fetch their playlists to let them create one
    let playlists = match watchers.is_empty() {
        true => {
            client
                .current_user_playlists()
                .try_collect::<Vec<_>>()
                .await?
        }
        false => vec![],
    };

    Ok(DashboardTemplate {
        name: user.id.to_string().split(':').last().unwrap().to_owned(),
        watchers,
        playlists,
    })
}
