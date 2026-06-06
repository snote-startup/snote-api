#![allow(unused)]

use futures_lite::Stream;
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::feature::project::model::{ChatMessage, ChatMessageCursor, ChatRole};

pub async fn create_chat_messages(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    role: ChatRole,
    content: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO chat_messages(project_id, role, content)
            VALUES($1, $2, $3)
        "#,
        project_id,
        role as _,
        content
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_chat_messages(
    executor: impl PgExecutor<'_>,
    project_id: Uuid,
    limit: usize,
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
    limit: usize,
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
