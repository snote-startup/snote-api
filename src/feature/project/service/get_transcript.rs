use sqlx::PgPool;
use uuid::Uuid;

use crate::feature::project::{model::TranscriptSegment, repository};

pub async fn get_transcript(
    database: &PgPool,
    id: Uuid,
) -> color_eyre::Result<Vec<TranscriptSegment>> {
    let transcripts = repository::get_transcript(database, id).await?;

    Ok(transcripts)
}
