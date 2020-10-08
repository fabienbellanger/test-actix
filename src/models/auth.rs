//! JWT module

use crate::config::Config;
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

static ONE_MONTH: i64 = 60 * 60 * 24 * 30; // In seconds

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub user_id: String,
    pub user_lastname: String,
    pub user_firstname: String,
    pub user_email: String,
}

pub struct JWT {}

impl JWT {
    // Generate JWT
    pub fn generate(
        user_id: String,
        user_lastname: String,
        user_firstname: String,
        user_email: String,
    ) -> Result<(String, i64), Box<dyn std::error::Error>> {
        let header = Header::new(Algorithm::HS512);
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = Claims {
            sub: user_id.clone(),
            exp: now + ONE_MONTH,
            iat: now,
            nbf: now,
            user_id: user_id,
            user_lastname: user_lastname,
            user_firstname: user_firstname,
            user_email: user_email,
        };

        let settings = Config::load()?;
        let secret_key = settings.jwt_secret_key;

        let token = encode(
            &header,
            &payload,
            &EncodingKey::from_secret(secret_key.as_bytes()),
        )?;

        Ok((token, now))
    }

    // Parse JWT
    pub fn parse(token: String) -> Result<Claims, Box<dyn std::error::Error>> {
        let validation = Validation::new(Algorithm::HS512);
        let settings = Config::load()?;
        let secret_key = settings.jwt_secret_key;

        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &validation,
        )?;

        Ok(token.claims)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_generate() {
        unimplemented!();
    }
}
