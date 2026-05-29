use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    config::CONFIG,
    util::token::{CompletedTokenUtil, TokenUtil},
};

pub struct ApiState {
    pub database: PgPool,
    pub token_util: CompletedTokenUtil,
}

impl ApiState {
    pub async fn new() -> color_eyre::Result<Arc<ApiState>> {
        let database = PgPool::connect(&CONFIG.database_url).await?;
        let token_util = CompletedTokenUtil {
            access: TokenUtil::new(&CONFIG.jwt_secret, CONFIG.jwt_expired_in),
            refresh: TokenUtil::new(&CONFIG.jwt_refresh_secret, CONFIG.jwt_refresh_expired_in),
        };

        Ok(Arc::new(ApiState {
            database,
            token_util,
        }))
    }
}
