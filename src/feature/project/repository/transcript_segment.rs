use pgvector::Vector;
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::feature::project::model::TranscriptSegment;

pub async fn create_transcript_segments(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    segments: &[TranscriptSegment],
) -> sqlx::Result<()> {
    let project_ids = vec![project_id; segments.len()];
    let speakers: Vec<_> = segments.iter().map(|x| x.speaker.clone()).collect();
    let texts: Vec<_> = segments.iter().map(|x| x.text.clone()).collect();
    let starts: Vec<_> = segments.iter().map(|x| x.start).collect();
    let ends: Vec<_> = segments.iter().map(|x| x.end).collect();

    sqlx::query!(
        r#"
            INSERT INTO transcript_segments(project_id, speaker, content, start_time, end_time)
            SELECT * FROM UNNEST(
                $1::uuid[],
                $2::text[],
                $3::text[],
                $4::int[],
                $5::int[]
            )
        "#,
        &project_ids,
        &speakers,
        &texts,
        &starts,
        &ends
    )
    .execute(executor)
    .await?;

    Ok(())
}

#[allow(unused)]
pub async fn create_transcript_segment(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    segment: &TranscriptSegment,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO transcript_segments(project_id, speaker, content, start_time, end_time)
            VALUES($1, $2, $3, $4, $5)
        "#,
        project_id,
        segment.speaker,
        segment.text,
        segment.start,
        segment.end,
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_transcript_segments(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
) -> sqlx::Result<Vec<TranscriptSegment>> {
    sqlx::query_as!(
        TranscriptSegment,
        r#"
            SELECT id, speaker, content as "text:_", start_time as "start:_", end_time as "end:_"
            FROM transcript_segments
            WHERE project_id = $1
            ORDER BY start_time, end_time
        "#,
        project_id
    )
    .fetch_all(executor)
    .await
}

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
