use crate::api::client2;

#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: String,
    pub token: rspotify::Token,
    pub client: rspotify::AuthCodeSpotify,
    pub client2: client2::Client,
}
