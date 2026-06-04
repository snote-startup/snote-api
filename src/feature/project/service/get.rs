use sqlx::PgPool;
use uuid::Uuid;

use crate::feature::project::{model::Project, repository};

pub async fn get(database: &PgPool, id: Uuid) -> color_eyre::Result<Option<Project>> {
    let project = repository::get_project(database, id).await?;

    Ok(project)
}
