use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::{Result, shared::token::model::Claims};

pub struct PartialTokenService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expired_in: u64,
}

impl PartialTokenService {
    pub fn new(secret: &str, expired_in: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expired_in,
        }
    }

    pub fn encode(&self, id: Uuid) -> Result<String> {
        let now = Utc::now().timestamp() as u64;
        let claims = Claims {
            sub: id,
            exp: now + self.expired_in,
        };

        let token = jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)?;

        Ok(token)
    }

    pub fn decode(&self, token: &str) -> color_eyre::Result<Uuid> {
        let token =
            jsonwebtoken::decode::<Claims>(token, &self.decoding_key, &Validation::default())?;

        Ok(token.claims.sub)
    }
}
