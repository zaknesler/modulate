use crate::web::context::ApiContext;
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
use r2d2_sqlite::rusqlite::params;
use rspotify::Token;
use std::sync::Arc;

#[allow(dead_code)]
pub async fn middleware<B>(
    State(ctx): State<Arc<ApiContext>>,
    mut req: Request<B>,
    next: Next<B>,
) -> crate::Result<impl IntoResponse> {
    match try_authorize(ctx).await {
        Ok(token) => {
            req.extensions_mut().insert(token);
            Ok(next.run(req).await)
        }
        Err(_) => Err(crate::error::Error::UnauthorizedError),
    }
}

async fn try_authorize(ctx: Arc<ApiContext>) -> crate::Result<Token> {
    // lazily get the first token
    // TODO: find token via cookie

    let token: String = ctx
        .db
        .get()?
        .prepare("SELECT token FROM TOKENS LIMIT 1")?
        .query_row(params![], |row| Ok(row.get(0)?))?;

    let token: Token = serde_json::from_str(&token)?;

    // TODO: check that the token has not expired, and refresh if it has

    Ok(token)
}
