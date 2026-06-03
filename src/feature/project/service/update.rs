use sqlx::PgPool;
use uuid::Uuid;

use crate::feature::project::repository;

pub async fn update(
    database: &PgPool,
    id: Uuid,
    account_id: Uuid,
    title: Option<&str>,
    description: Option<&str>,
) -> color_eyre::Result<()> {
    repository::update_project(database, id, account_id, title, description, None, None).await?;

    Ok(())
}
