use sqlx::PgExecutor;
use uuid::Uuid;

use crate::feature::chat::model::{ChatMessage, ChatMessageCursor, ChatRole};

pub async fn create_chat_messages(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    roles: &[ChatRole],
    contents: &[String],
) -> sqlx::Result<()> {
    let project_ids = vec![project_id; roles.len()];

    sqlx::query!(
        r#"
            INSERT INTO chat_messages(project_id, role, content)
            SELECT * FROM UNNEST(
                $1::uuid[],
                $2::chat_role[],
                $3::text[]
            )
        "#,
        &project_ids,
        roles as _,
        contents
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_chat_messages(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    limit: u32,
) -> sqlx::Result<Vec<ChatMessage>> {
    sqlx::query_as!(
        ChatMessage,
        r#"
            SELECT id, role as "role: _", content, created_at
            FROM chat_messages
            WHERE project_id = $1
            ORDER BY created_at DESC, id DESC
            LIMIT $2
        "#,
        project_id,
        limit as i64,
    )
    .fetch_all(executor)
    .await
}

pub async fn get_paginated_chat_messages(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    cursor: ChatMessageCursor,
    limit: u32,
) -> sqlx::Result<Vec<ChatMessage>> {
    sqlx::query_as!(
        ChatMessage,
        r#"
            SELECT id, role as "role: _", content, created_at
            FROM chat_messages
            WHERE project_id = $1 AND(
                created_at < $2
                OR (created_at = $2 AND id < $3)
            )
            ORDER BY created_at DESC, id DESC
            LIMIT $4
        "#,
        project_id,
        cursor.created_at,
        cursor.id,
        limit as i64
    )
    .fetch_all(executor)
    .await
}
