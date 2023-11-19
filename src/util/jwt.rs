use crate::error::Error;
use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

const JWT_EXPIRATION_DAYS: i64 = 90;

pub const JWT_CLAIM_USER: &str = "sub";
pub const JWT_CLAIM_ISSUED: &str = "iat";
pub const JWT_CLAIM_EXPIRATION: &str = "exp";

/// Create a JWT for the given UUID
pub fn sign_jwt(secret: &str, user_id: String) -> crate::Result<String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let now: DateTime<Utc> = Utc::now();

    let mut claims = BTreeMap::new();
    claims.insert(JWT_CLAIM_USER, user_id);
    claims.insert(JWT_CLAIM_ISSUED, now.to_rfc3339());
    claims.insert(
        JWT_CLAIM_EXPIRATION,
        (now + Duration::days(JWT_EXPIRATION_DAYS)).to_rfc3339(),
    );

    claims.sign_with_key(&key).map_err(|err| err.into())
}

/// Extract the claims from a valid JWT
pub fn extract_claims(secret: &str, jwt: &str) -> crate::Result<BTreeMap<String, String>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    jwt.verify_with_key(&key).map_err(|err| err.into())
}

/// Verify that a JWT is valid and extract the user's ID
pub fn verify_jwt(secret: &str, jwt: &str) -> crate::Result<String> {
    let claims = extract_claims(secret, jwt)?;

    // Check that the token hasn't expired
    if claims
        .get(JWT_CLAIM_EXPIRATION)
        .ok_or_else(|| Error::InvalidJwt)?
        .parse::<DateTime<Utc>>()?
        < Utc::now()
    {
        return Err(Error::JwtExpired);
    }

    // Attempt to extract the user's ID
    Ok(claims
        .get(JWT_CLAIM_USER)
        .ok_or_else(|| Error::InvalidJwt)?
        .to_owned())
}
