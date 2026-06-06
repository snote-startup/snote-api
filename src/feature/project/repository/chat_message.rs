#![allow(unused)]

use futures_lite::Stream;
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::feature::project::model::{ChatMessage, ChatRole};

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

pub fn get_chat_messages<'a>(
    executor: impl PgExecutor<'a> + 'a,
    project_id: Uuid,
) -> impl Stream<Item = sqlx::Result<ChatMessage>> {
    sqlx::query_as!(
        ChatMessage,
        r#"
            SELECT role as "role: _", content, created_at
            FROM chat_messages
            WHERE project_id = $1
            ORDER BY created_at DESC
        "#,
        project_id
    )
    .fetch(executor)
}
