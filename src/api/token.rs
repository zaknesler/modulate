use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub expires_in: chrono::Duration,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token: String,
    pub scopes: HashSet<String>,
}
