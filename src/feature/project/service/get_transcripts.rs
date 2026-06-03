use sqlx::PgPool;
use uuid::Uuid;

use crate::feature::project::{model::Transcript, repository};

pub async fn get_transcripts(
    database: &PgPool,
    project_id: Uuid,
) -> color_eyre::Result<Vec<Transcript>> {
    let transcripts = repository::get_transcripts(database, project_id).await?;

    Ok(transcripts)
}
