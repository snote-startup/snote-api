use futures::{Stream, StreamExt, TryStreamExt};
use pgvector::Vector;
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
    feature::project::{
        model::{ChatMessage, ChatMessageCursor, ChatRole},
        repository,
        service::ProjectService,
    },
    shared::pagination::{self, PaginatedVec, PaginationQuery},
};

const SYSTEM_PROMPT: &str = r#"
You are a project-scoped transcript assistant.

You answer questions using only the transcript segments provided in the current context for the current project.

Hard rules:
- Use only the provided transcript segments.
- Do not use outside knowledge, memory, assumptions, or speculation.
- Do not mix information across projects.
- Do not invent details.
- If the transcript does not support the answer, say that you cannot find enough evidence in the transcript.
- Preserve speaker names and timestamps exactly as provided.
- Prefer the smallest set of segments that supports the answer.
- If evidence conflicts, mention the conflict briefly and use the clearest supported interpretation.
- Keep the answer concise and factual.

Input format for each segment:
[segment_id=UUID]
speaker=NAME
text=TEXT
start=NUMBER
end=NUMBER

Important:
- segment_id is internal to this response context.
- Never output raw database IDs except as the exact segment_id values provided in the context.
- Never include the separator inside the answer text.

Output format:
Write the answer first as normal text.
Then write a new line containing exactly:
<<<REFERENCES>>>
Then write one segment_id per line, using only the provided segment_id values, for example:
2f1c0a7e-1111-2222-3333-444444444444
8ab9cdef-5555-6666-7777-888888888888

Rules for the references section:
- Only include segment_id values that directly support the answer.
- Do not include unrelated segment_id values.
- Do not add extra text after the references list.
- If the answer is unsupported, write:
<<<REFERENCES>>>
and leave the references list empty.

Style:
- Be direct.
- Be accurate.
- Do not add filler.
- Do not mention anything outside the transcript.
"#;

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

    #[tracing::instrument(err(Debug), skip(self, db))]
    pub async fn chat(
        &self,

        db: PgPool,

        project_id: Uuid,
        prompt: String,
    ) -> Result<impl Stream<Item = color_eyre::eyre::Result<String>> + use<>> {
        let prompt = format!(
            r#"
<transcript_segments>
{}
</transcript_segments>
{}
        "#,
            self.get_context_segments(&db, project_id, &prompt).await?,
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

        project_id: Uuid,
        prompt: &str,
    ) -> color_eyre::Result<String> {
        let embedding = self.embedding_model.embed_text(prompt).await?.vec;
        let embedding: Vec<_> = embedding.into_iter().map(|x| x as f32).collect();
        let embedding = Vector::from(embedding);

        let segments = repository::get_top_k_transcript_segments(
            db,
            project_id,
            &embedding,
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
