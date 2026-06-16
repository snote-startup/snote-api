use pgvector::Vector;
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::feature::chat::model::TranscriptSegment;

pub async fn get_top_k_transcript_segments(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    embedding: &Vector,
    k: u32,
) -> sqlx::Result<Vec<TranscriptSegment>> {
    sqlx::query_as!(
        TranscriptSegment,
        r#"
            SELECT id, speaker, content as "text:_", start_time as "start:_", end_time as "end:_"
            FROM transcript_segments
            WHERE project_id = $1
            ORDER BY embedding <=> $2
            LIMIT $3
        "#,
        project_id,
        embedding as _,
        k as i64
    )
    .fetch_all(executor)
    .await
}
