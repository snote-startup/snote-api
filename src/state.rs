use sqlx::PgPool;

use crate::{
    config::Config,
    shared::{storage::service::StorageService, token::service::TokenService},
};

pub struct AppState {
    pub config: Config,

    pub database: PgPool,

    pub token_service: TokenService,
    pub storage_service: StorageService,
}

impl AppState {
    pub async fn new(config: Config) -> color_eyre::Result<AppState> {
        Ok(AppState {
            database: PgPool::connect(&config.database_url).await?,

            token_service: TokenService::new(&config),
            storage_service: StorageService::new(&config).await?,

            config,
        })
    }
}
