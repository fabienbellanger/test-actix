//! JWT module

use crate::config::Config;
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub user_id: String,
    pub user_lastname: String,
    pub user_firstname: String,
}

pub struct JWT {}

impl JWT {
    // Generate JWT
    pub fn generate(
        user_id: String,
        user_lastname: String,
        user_firstname: String,
    ) -> Result<String, Error> {
        let header = Header::new(Algorithm::HS512);
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = Claims {
            sub: "Test".to_owned(), // TODO: Change !
            exp: now + ONE_WEEK,
            iat: now,
            nbf: now,
            user_id: user_id,
            user_lastname: user_lastname,
            user_firstname: user_firstname,
        };

        // TODO: Faire mieux que charger tout le fichier !
        let settings = Config::load().expect("Cannot find .env file");
        let secret_key = settings.jwt_secret_key;

        let token = encode(
            &header,
            &payload,
            &EncodingKey::from_secret(secret_key.as_bytes()),
        )?;

        Ok(token)
    }
}
