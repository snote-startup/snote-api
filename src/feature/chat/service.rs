use futures::{Stream, StreamExt, TryStreamExt};
use rig_core::{
    agent::{Agent, MultiTurnStreamItem},
    client::{CompletionClient, EmbeddingsClient},
    embeddings::EmbeddingModel,
    message::Message,
    providers::gemini,
    streaming::{StreamedAssistantContent, StreamingChat},
};
use sqlx::PgPool;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    error::Result,
    feature::{
        chat::{
            SYSTEM_PROMPT,
            model::{ChatMessage, ChatMessageCursor, ChatRole},
            repository,
        },
        project::service::ProjectService,
    },
    shared::pagination::{self, PaginatedVec, PaginationQuery},
};

pub struct ChatService {
    pub embedding_model: gemini::EmbeddingModel,
    pub agent: Agent<gemini::CompletionModel>,
    pub context_transcript_size: u32,
    pub context_history_size: u32,
}

impl ChatService {
    #[tracing::instrument(err(Debug))]
    pub fn new(
        api_key: &str,
        context_transcript_size: u32,
        context_history_size: u32,
    ) -> color_eyre::Result<Self> {
        let client = gemini::Client::new(api_key)?;

        let embedding_model = client.embedding_model(gemini::EMBEDDING_001);

        let agent = client
            .agent(gemini::completion::GEMINI_3_FLASH_PREVIEW)
            .preamble(SYSTEM_PROMPT)
            .build();

        Ok(Self {
            embedding_model,
            agent,
            context_transcript_size,
            context_history_size,
        })
    }

    #[tracing::instrument(err(Debug), skip(self, db))]
    pub async fn get_history(
        &self,

        db: &PgPool,

        project_id: Uuid,
        //TODO: check ownership
        #[allow(unused)] account_id: Uuid,
        query: PaginationQuery,
    ) -> Result<PaginatedVec<ChatMessage>> {
        let mut data = match query.cursor {
            Some(cursor) => {
                let cursor: ChatMessageCursor = pagination::decode(&cursor)?;
                repository::get_paginated_chat_messages(db, project_id, cursor, query.limit + 1)
                    .await
            }
            None => repository::get_chat_messages(db, project_id, query.limit + 1).await,
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

    #[tracing::instrument(err(Debug), skip(self, db, project_svc))]
    pub async fn chat(
        &self,

        db: PgPool,
        project_svc: &ProjectService,

        project_id: Uuid,
        account_id: Uuid,
        prompt: String,
    ) -> Result<impl Stream<Item = color_eyre::eyre::Result<String>> + use<>> {
        let prompt = format!(
            r#"
<transcript_segments>
{}
</transcript_segments>
{}
        "#,
            self.get_context_segments(&db, project_svc, project_id, account_id, &prompt)
                .await?,
            prompt
        );
        let history = self.get_context_history(&db, project_id).await?;

        let stream = self.agent.stream_chat(&prompt, history).await;

        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        tokio::spawn(async move {
            let Some(response) = rx.recv().await else {
                tracing::error!("No response from llm");
                return;
            };

            if let Err(error) = repository::create_chat_messages(
                &db,
                project_id,
                &[ChatRole::User, ChatRole::Assistant],
                &[prompt, response],
            )
            .await
            {
                tracing::error!(?error, "Failed to update history");
            }
        });

        let stream = stream
            .inspect_ok(move |chunk| {
                if let MultiTurnStreamItem::FinalResponse(final_response) = chunk {
                    let response = final_response.response();
                    let _ = tx.send(response.to_string());
                }
            })
            .filter_map(|item| async move {
                match item {
                    Ok(MultiTurnStreamItem::StreamAssistantItem(
                        StreamedAssistantContent::Text(text),
                    )) => Some(Ok(text.text)),
                    Err(error) => Some(Err(error.into())),
                    _ => None,
                }
            });
        Ok(stream)
    }

    async fn get_context_segments(
        &self,

        db: &PgPool,
        project_svc: &ProjectService,

        project_id: Uuid,
        account_id: Uuid,
        prompt: &str,
    ) -> Result<String> {
        let embedding = self.embedding_model.embed_text(prompt).await?.vec;
        let embedding: Vec<_> = embedding.into_iter().map(|x| x as f32).collect();

        let segments = project_svc
            .get_top_k_transcript_segments(
                db,
                account_id,
                project_id,
                embedding,
                self.context_transcript_size,
            )
            .await?;
        let mut context = String::new();
        for segment in segments {
            context.push_str(&segment.to_string());
            context.push('\n');
        }

        Ok(context)
    }

    async fn get_context_history(
        &self,

        db: &PgPool,

        project_id: Uuid,
    ) -> color_eyre::Result<Vec<Message>> {
        let history =
            repository::get_chat_messages(db, project_id, self.context_history_size).await?;

        Ok(history
            .into_iter()
            .map(|m| match m.role {
                ChatRole::User => Message::user(m.content),
                ChatRole::Assistant => Message::assistant(m.content),
            })
            .collect())
    }
}
