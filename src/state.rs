use sqlx::PgPool;

use crate::{
    config::CONFIG,
    util::token::{CompletedTokenUtil, TokenUtil},
};

pub struct AppState {
    pub database: PgPool,
    pub s3: aws_sdk_s3::Client,
    pub token_util: CompletedTokenUtil,
}

impl AppState {
    pub async fn new() -> color_eyre::Result<AppState> {
        let database = PgPool::connect(&CONFIG.database_url).await?;

        let s3_config = aws_config::load_from_env().await;
        let s3 = aws_sdk_s3::Client::new(&s3_config);

        let token_util = CompletedTokenUtil {
            access: TokenUtil::new(&CONFIG.jwt_secret, CONFIG.jwt_expired_in),
            refresh: TokenUtil::new(&CONFIG.jwt_refresh_secret, CONFIG.jwt_refresh_expired_in),
        };

        Ok(AppState {
            database,
            s3,
            token_util,
        })
    }
}
