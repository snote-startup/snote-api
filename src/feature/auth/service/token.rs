use chrono::Utc;
use jsonwebtoken::{Header, Validation};
use uuid::Uuid;

use crate::error::Result;
use crate::feature::auth::model::{Claims, TokenMetadata, TokenPair};

pub fn create_token_pair(
    access_metadata: &TokenMetadata,
    refresh_metadata: &TokenMetadata,
    id: Uuid,
) -> Result<TokenPair> {
    let access = create_token(access_metadata, id)?;
    let refresh = create_token(refresh_metadata, id)?;

    Ok(TokenPair { access, refresh })
}

fn create_token(metadata: &TokenMetadata, id: Uuid) -> Result<String> {
    let now = Utc::now().timestamp() as u64;
    let claims = Claims {
        sub: id,
        exp: now + metadata.expired_in,
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &metadata.encoding_key)?;

    Ok(token)
}

pub fn decode_token(metadata: &TokenMetadata, token: &str) -> Result<Uuid> {
    let claims =
        jsonwebtoken::decode::<Claims>(token, &metadata.decoding_key, &Validation::default())?
            .claims;

    Ok(claims.sub)
}
