use super::error::WebError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        // If we have a specific status and error message to respond with, use it, otherwise just return a generic 500
        let (status, error) = handle_error_response(&self).unwrap_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Value::String(self.to_string()),
            )
        });

        let data = Json(json!({ "status": status.as_u16(), "error": error }));
        (status, data).into_response()
    }
}

/// Create a response for a specific error type
fn handle_error_response(error: &WebError) -> Option<(StatusCode, Value)> {
    Some(match error {
        WebError::NotFoundError => (StatusCode::NOT_FOUND, Value::String(error.to_string())),
        WebError::DbError(outer) => match outer {
            crate::db::error::DbError::SQLiteError(inner) => match inner {
                rusqlite::Error::QueryReturnedNoRows => (
                    StatusCode::NOT_FOUND,
                    Value::String(WebError::NotFoundError.to_string()),
                ),
                _ => return None,
            },
            _ => return None,
        },
        WebError::ClientError(outer) => match outer {
            crate::api::error::ClientError::ApiError { status, message } => (
                axum::http::StatusCode::from_u16(*status)
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                Value::String(message.clone()),
            ),
            _ => return None,
        },
        WebError::UnauthorizedError | WebError::JwtExpiredError | WebError::JwtInvalidError => {
            (StatusCode::UNAUTHORIZED, Value::String(error.to_string()))
        }
        WebError::InvalidFormData(err) => {
            (StatusCode::UNPROCESSABLE_ENTITY, Value::String(err.clone()))
        }
        WebError::ValidationErrors(err) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            json!({ "fields": err.field_errors() }),
        ),
        WebError::ValidationError(err) => (StatusCode::UNPROCESSABLE_ENTITY, json!(err)),
        _ => return None,
    })
}
