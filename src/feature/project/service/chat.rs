use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::Result,
    feature::project::{
        model::{ChatMessage, ChatMessageCursor},
        repository,
    },
    shared::pagination::{
        self,
        model::{PaginatedVec, PaginationQuery},
    },
};

#[tracing::instrument(err(Debug), skip(database))]
pub async fn get_chat_messages(
    database: &PgPool,
    account_id: Uuid,
    id: Uuid,
    query: PaginationQuery,
) -> Result<PaginatedVec<ChatMessage>> {
    let mut data = match query.cursor {
        Some(cursor) => {
            let cursor: ChatMessageCursor = pagination::service::decode(&cursor)?;
            repository::get_paginated_chat_messages(database, id, cursor, query.limit + 1).await
        }
        None => repository::get_chat_messages(database, id, query.limit + 1).await,
    }?;
    let next_cursor = if data.len() > query.limit as usize {
        let last = data.pop().unwrap();
        Some(pagination::service::encode(&ChatMessageCursor {
            id: last.id,
            created_at: last.created_at,
        })?)
    } else {
        None
    };

    Ok(PaginatedVec { data, next_cursor })
}

#[tracing::instrument(err(Debug), skip(database))]
pub async fn chat(
    database: &PgPool,
    account_id: Uuid,
    id: Uuid,
    query: PaginationQuery,
) -> Result<PaginatedVec<ChatMessage>> {
    todo!()
}
