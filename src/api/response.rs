use serde::Deserialize;
use std::fmt::Debug;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SpotifyResponse<T> {
    Success(T),
    Error(SpotifyErrorWrapper),
}

#[derive(Debug, Deserialize)]
pub struct SpotifyErrorWrapper {
    pub error: SpotifyErrorResponse,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyErrorResponse {
    pub status: u16,
    pub message: String,
}

impl From<SpotifyErrorWrapper> for super::error::ClientError {
    fn from(value: SpotifyErrorWrapper) -> Self {
        Self::ApiError {
            status: value.error.status,
            message: value.error.message,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SnapshotResponse {
    pub snapshot_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T>
where
    T: Debug,
{
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
    pub href: String,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub items: Vec<T>,
}
