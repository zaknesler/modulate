use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse, TokenResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::error::ClientError;

/// Number of seconds to subtract from expires_at to ensure we have enough time to check that an access_token is valid
/// e.g. Spotify's access tokens are valid for 60 minutes, so settings this to 60 seconds makes them valid for 59 minutes
const EXPIRATION_OFFSET_SECONDS: i64 = 60;

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub expires_in: chrono::Duration,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token: Option<String>,
    pub scopes: HashSet<String>,
}

impl Token {
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() >= self.expires_at
    }
}

impl TryFrom<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> for Token {
    type Error = ClientError;

    fn try_from(
        res: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<Self, Self::Error> {
        let scopes = res.scopes().expect("spotify returns this").clone();

        let expires_in =
            chrono::Duration::from_std(res.expires_in().expect("spotify returns this"))?;

        // Set the expiration to 1 minute before the "expires_in" duration returned from Spotify
        let expires_at =
            chrono::Utc::now() + expires_in - chrono::Duration::seconds(EXPIRATION_OFFSET_SECONDS);

        Ok(Token {
            access_token: res.access_token().secret().to_string(),
            expires_in,
            expires_at,
            refresh_token: res.refresh_token().map(|token| token.secret().clone()),
            scopes: HashSet::from_iter(scopes.iter().map(|scope| scope.to_string())),
        })
    }
}
