use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    id: String,
    uri: String,
    name: String,
    snapshot_id: String,
    images: Vec<Image>,
    external_urls: ExternalUrls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    url: String,
    width: Option<i32>,
    height: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}
