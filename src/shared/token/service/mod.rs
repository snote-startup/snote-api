mod partial;

pub use partial::*;
use uuid::Uuid;

use crate::{config::Config, error::Result, shared::token::model::TokenPair};

pub struct TokenService {
    pub access: PartialTokenService,
    pub refresh: PartialTokenService,
}

impl TokenService {
    pub fn new(config: &Config) -> Self {
        TokenService {
            access: PartialTokenService::new(&config.jwt_secret, config.jwt_expired_in),
            refresh: PartialTokenService::new(
                &config.jwt_refresh_secret,
                config.jwt_refresh_expired_in,
            ),
        }
    }

    pub fn encode(&self, id: Uuid) -> Result<TokenPair> {
        Ok(TokenPair {
            refresh: self.refresh.encode(id)?,
            access: self.access.encode(id)?,
        })
    }
}
