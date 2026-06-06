use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    feature::project::{
        model::{ChatMessage, ChatMessageCursor},
        repository,
    },
    util::pagination::{self, PaginatedVec, PaginationQuery},
};

pub async fn get_chat_messages(
    database: &PgPool,
    id: Uuid,
    query: PaginationQuery,
) -> color_eyre::Result<PaginatedVec<ChatMessage>> {
    let mut data = match query.cursor {
        Some(cursor) => {
            let cursor: ChatMessageCursor = pagination::decode(&cursor)?;
            repository::get_paginated_chat_messages(database, id, cursor, query.limit + 1).await
        }
        None => repository::get_chat_messages(database, id, query.limit + 1).await,
    }?;
    let next_cursor = if data.len() > query.limit as usize {
        let last = data.pop().unwrap();
        Some(pagination::encode(&ChatMessageCursor {
            id: last.id,
            created_at: last.created_at,
        })?)
    } else {
        None
    };

    Ok(PaginatedVec { data, next_cursor })
}
