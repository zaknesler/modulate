use super::id::{PlaylistId, SnapshotId, TrackId, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub display_name: String,
    pub images: Vec<Image>,
    pub external_urls: ExternalUrls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackPartial {
    pub id: TrackId,
    #[serde(rename = "type")]
    pub kind: TrackType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrackType {
    Episode,
    Track,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistPartial {
    pub id: PlaylistId,
    pub name: String,
    pub snapshot_id: SnapshotId,
    pub images: Vec<Image>,
    pub external_urls: ExternalUrls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}
