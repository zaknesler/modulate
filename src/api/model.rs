use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub uri: String,
    pub display_name: String,
    pub images: Vec<Image>,
    pub external_urls: ExternalUrls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub uri: String,
    pub name: String,
    pub snapshot_id: String,
    pub images: Vec<Image>,
    pub external_urls: ExternalUrls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}
