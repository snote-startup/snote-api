use itertools::MultiUnzip;
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::feature::task::model::{MinimalTask, Task, TaskPriority, TaskStatus};

pub async fn create_tasks(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    tasks: Vec<MinimalTask>,
) -> sqlx::Result<()> {
    let project_ids = vec![project_id; tasks.len()];
    let (statuses, priorities, contents): (Vec<_>, Vec<_>, Vec<_>) = tasks
        .into_iter()
        .map(|t| (t.status, t.priority, t.content))
        .multiunzip();

    sqlx::query!(
        r#"
            INSERT INTO tasks(project_id, status, priority, content)
            SELECT * FROM UNNEST(
                $1::uuid[],
                $2::task_status[],
                $3::task_priority[],
                $4::text[]
            )
        "#,
        &project_ids,
        &statuses as _,
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
