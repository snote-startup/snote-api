use std::fmt::Write;

use futures::{Stream, StreamExt, TryStreamExt};
use pgvector::Vector;
use rig_core::message::Message;
use rig_core::{
    agent::MultiTurnStreamItem,
    client::{CompletionClient, EmbeddingsClient},
    embeddings::EmbeddingModel,
    providers::gemini,
    streaming::{StreamedAssistantContent, StreamingChat},
};
use sqlx::PgPool;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    config::CONFIG,
    feature::project::{model::ChatRole, repository},
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

pub async fn chat(
    database: PgPool,
    id: Uuid,
    prompt: String,
) -> color_eyre::Result<impl Stream<Item = color_eyre::Result<String>>> {
    let client = gemini::Client::new(&CONFIG.gemini_api_key)?;

    let completed_prompt = build_completed_prompt(&database, &client, id, &prompt).await?;

    let history = get_history(&database, id).await?;

    let agent = client
        .agent(gemini::completion::GEMINI_3_FLASH_PREVIEW)
        .preamble(SYSTEM_PROMPT)
        .build();
    let stream = agent.stream_chat(completed_prompt, history).await;

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    tokio::spawn(async move {
        let Some(response) = rx.recv().await else {
            tracing::error!("No response from llm");
            return;
        };

        if let Err(error) = repository::create_chat_messages(
            &database,
            id,
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
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                    text,
                ))) => Some(Ok(text.text)),
                Err(error) => Some(Err(color_eyre::eyre::Error::from(error))),
                _ => None,
            }
        });
    Ok(stream)
}

async fn build_completed_prompt(
    database: &PgPool,
    client: &gemini::Client,
    id: Uuid,
    prompt: &str,
) -> color_eyre::Result<String> {
    let embedding_client = client.embedding_model(gemini::EMBEDDING_001);

    let embedding = embedding_client.embed_text(prompt).await?.vec;
    let embedding: Vec<_> = embedding.into_iter().map(|x| x as f32).collect();
    let embedding = Vector::from(embedding);

    let segments = repository::get_top_k_transcript_segments(
        database,
        id,
        &embedding,
        CONFIG.chat_context_transcript_size,
    )
    .await?;
    let mut context = String::new();
    for segment in &segments {
        writeln!(&mut context, "[segment_id={}]", segment.id).unwrap();
        writeln!(&mut context, "speaker={}", segment.speaker).unwrap();
        writeln!(&mut context, "text={}", segment.text).unwrap();
        writeln!(&mut context, "start={}", segment.start).unwrap();
        writeln!(&mut context, "end={}", segment.end).unwrap();
        context.push('\n');
    }

    Ok(format!(
        r#"
<context>
{}
</context>
{}
        "#,
        context, prompt
    ))
}

async fn get_history(database: &PgPool, id: Uuid) -> color_eyre::Result<Vec<Message>> {
    let history =
        repository::get_chat_messages(database, id, CONFIG.chat_context_history_size).await?;
    let history: Vec<_> = history
        .into_iter()
        .map(|m| match m.role {
            ChatRole::User => Message::user(m.content),
            ChatRole::Assistant => Message::assistant(m.content),
        })
        .collect();

    Ok(history)
}
