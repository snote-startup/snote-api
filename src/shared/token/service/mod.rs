mod partial;

pub use partial::*;

use crate::config::Config;

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
}
