use sqlx::PgPool;
use uuid::Uuid;

use crate::feature::project::repository;

#[tracing::instrument(err(Debug), skip(database))]
pub async fn create(
    database: &PgPool,
    account_id: Uuid,
    title: &str,
    description: Option<&str>,
) -> color_eyre::Result<Uuid> {
    let id = repository::create_project(database, account_id, title, description).await?;

    Ok(id)
}
