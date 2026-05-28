use std::sync::Arc;

use sqlx::PgPool;

use crate::config::CONFIG;

pub struct ApiState {
    pub database: PgPool,
}

impl ApiState {
    pub async fn new() -> color_eyre::Result<Arc<ApiState>> {
        let database = PgPool::connect(&CONFIG.database_url).await?;

        Ok(Arc::new(ApiState { database }))
    }
}
