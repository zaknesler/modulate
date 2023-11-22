#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: String,
    pub token: rspotify::Token,
    pub client: rspotify::AuthCodeSpotify,
}
