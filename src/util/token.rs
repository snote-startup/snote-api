use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct CompletedTokenUtil {
    pub access: TokenUtil,
    pub refresh: TokenUtil,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: u64,
}

pub struct TokenUtil {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expired_in: u64,
}

impl TokenUtil {
    pub fn new(secret: &str, expired_in: u64) -> TokenUtil {
        TokenUtil {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expired_in,
        }
    }

    pub fn encode(&self, id: Uuid) -> color_eyre::Result<String> {
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
