use sqlx::PgExecutor;
use uuid::Uuid;

use crate::feature::project::model::Transcript;

pub async fn create_transcripts(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    transcripts: &[Transcript],
) -> sqlx::Result<()> {
    let project_ids = vec![project_id; transcripts.len()];
    let speakers: Vec<_> = transcripts.iter().map(|x| x.speaker.clone()).collect();
    let texts: Vec<_> = transcripts.iter().map(|x| x.text.clone()).collect();
    let starts: Vec<_> = transcripts.iter().map(|x| x.start as i32).collect();
    let ends: Vec<_> = transcripts.iter().map(|x| x.end as i32).collect();

    sqlx::query!(
        r#"
            INSERT INTO transcripts(project_id, speaker, content, start_time, end_time)
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

pub async fn create_transcript(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    transcript: &Transcript,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO transcripts(project_id, speaker, content, start_time, end_time)
            VALUES($1, $2, $3, $4, $5)
        "#,
        project_id,
        transcript.speaker,
        transcript.text,
        transcript.start,
        transcript.end,
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_transcripts(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
) -> sqlx::Result<Vec<Transcript>> {
    sqlx::query_as!(
        Transcript,
        r#"
            SELECT speaker, content as "text:_", start_time as "start:_", end_time as "end:_"
            FROM transcripts
            WHERE project_id = $1
            ORDER BY start_time, end_time
        "#,
        project_id
    )
    .fetch_all(executor)
    .await
}
