use crate::api::{client::Client, token::Token};

#[derive(Debug, Clone)]
pub struct Session {
    pub user_uri: String,
    pub token: Token,
    pub client: Client,
}
