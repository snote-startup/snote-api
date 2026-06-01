use sqlx::PgExecutor;
use uuid::Uuid;

use crate::feature::project::model::Project;

pub async fn create_project(
    executor: impl PgExecutor<'_>,
    account_id: Uuid,
    title: &str,
    description: Option<&str>,
) -> sqlx::Result<Uuid> {
    sqlx::query_scalar!(
        r#"
            INSERT INTO projects(account_id, title, description)
            VALUES($1, $2, $3)
            RETURNING id;
        "#,
        account_id,
        title,
        description
    )
    .fetch_one(executor)
    .await
}

pub async fn get_projects_by_account(
    executor: impl PgExecutor<'_>,
    account_id: Uuid,
) -> sqlx::Result<Vec<Project>> {
    sqlx::query_as!(
        Project,
        r#"
            SELECT id, title, description, audio_url
            FROM projects
            WHERE account_id = $1
        "#,
        account_id
    )
    .fetch_all(executor)
    .await
}

pub async fn get_project(
    executor: impl PgExecutor<'_>,
    id: Uuid,
    account_id: Uuid,
) -> sqlx::Result<Option<Project>> {
    sqlx::query_as!(
        Project,
        r#"
            SELECT id, title, description, audio_url
            FROM projects
            WHERE id = $1 AND account_id = $2
        "#,
        id,
        account_id
    )
    .fetch_optional(executor)
    .await
}

pub async fn update_project(
    executor: impl PgExecutor<'_>,
    id: Uuid,
    account_id: Uuid,
    title: Option<&str>,
    description: Option<&str>,
    audio_url: Option<&str>,
    transcript_ai_id: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            UPDATE projects
            SET title = COALESCE($3, title),
                description = COALESCE($4, description),
                audio_url = COALESCE($5, audio_url),
                transcript_ai_id = COALESCE($6, transcript_ai_id),
                updated_at = now()
            WHERE id = $1 AND account_id = $2
        "#,
        id,
        account_id,
        title,
        description,
        audio_url,
        transcript_ai_id
    )
    .execute(executor)
    .await?;

    Ok(())
}
