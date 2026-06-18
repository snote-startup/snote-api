use itertools::MultiUnzip;
use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::{MinimalTask, Task, TaskPriority, TaskStatus};

pub async fn create_tasks(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    tasks: Vec<MinimalTask>,
) -> sqlx::Result<()> {
    let project_ids = vec![project_id; tasks.len()];
    let (priorities, contents): (Vec<_>, Vec<_>) = tasks
        .into_iter()
        .map(|t| (t.priority, t.content))
        .multiunzip();

    sqlx::query!(
        r#"
            INSERT INTO tasks(project_id, priority, content)
            SELECT * FROM UNNEST(
                $1::uuid[],
                $2::task_priority[],
                $3::text[]
            )
        "#,
        &project_ids,
        &priorities as _,
        &contents
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_tasks_by_project(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
) -> sqlx::Result<Vec<Task>> {
    sqlx::query_as!(
        Task,
        r#"
            SELECT id, status as "status: _", priority as "priority: _", content, created_at
            FROM tasks
            WHERE project_id = $1
            ORDER BY created_at
        "#,
        project_id
    )
    .fetch_all(executor)
    .await
}

pub async fn update_task(
    executor: impl PgExecutor<'_>,
    id: Uuid,
    status: Option<TaskStatus>,
    priority: Option<TaskPriority>,
    content: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            UPDATE tasks
            SET status = COALESCE($2, status),
                priority = COALESCE($3, priority),
                content = COALESCE($4, content)
            WHERE id = $1
        "#,
        id,
        status as _,
        priority as _,
        content
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn delete_task(executor: impl PgExecutor<'_>, id: Uuid) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM tasks
            WHERE id = $1
        "#,
        id
    )
    .execute(executor)
    .await?;

    Ok(())
}
