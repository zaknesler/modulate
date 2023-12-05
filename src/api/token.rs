pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: chrono::Duration,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token: String,
    pub scope: Vec<String>,
}
