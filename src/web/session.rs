use crate::{
    api::client::{Client, WithToken},
    db::model::user::User,
};

#[derive(Debug, Clone)]
pub struct Session {
    pub client: Client<WithToken>,
    pub user: User,
}
