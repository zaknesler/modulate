use crate::{api::client::Client, db::model::user::User};

#[derive(Debug, Clone)]
pub struct Session {
    pub client: Client,
    pub user: User,
}
