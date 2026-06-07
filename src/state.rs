use sqlx::PgPool;

use crate::{
    config::Config,
    shared::{storage::service::S3Service, token::service::TokenService},
};

pub struct AppState {
    pub database: PgPool,

    pub token_service: TokenService,
    pub s3_service: S3Service,
}

impl AppState {
    pub async fn new(config: &Config) -> color_eyre::Result<AppState> {
        Ok(AppState {
            database: PgPool::connect(&config.database_url).await?,

            token_service: TokenService::new(config),
            s3_service: S3Service::new(config).await?,
        })
    }
}
