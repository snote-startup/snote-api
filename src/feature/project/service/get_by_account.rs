use sqlx::PgPool;
use uuid::Uuid;

use crate::feature::project::{model::Project, repository};

pub async fn get_by_account(
    database: &PgPool,
    account_id: Uuid,
) -> color_eyre::Result<Vec<Project>> {
    let projects = repository::get_projects_by_account(database, account_id).await?;

    Ok(projects)
}
