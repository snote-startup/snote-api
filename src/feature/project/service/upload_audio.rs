use aws_sdk_s3::primitives::ByteStream;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    feature::project::{external::assembly_ai, repository},
    util::storage,
};

pub async fn upload_audio(
    database: &PgPool,
    s3: &aws_sdk_s3::Client,
    id: Uuid,
    account_id: Uuid,
    content: ByteStream,
) -> color_eyre::Result<()> {
    let key = generate_key(id);
    let audio_url = storage::upload(s3, key, content).await?;

    let raw = assembly_ai::transcript::create(&audio_url).await?;
    let transcript_ai_id = raw.id;
    let transcripts = raw.transcript;

    let mut transaction = database.begin().await?;

    repository::update_project(
        &mut *transaction,
        id,
        account_id,
        None,
        None,
        Some(&audio_url),
        Some(&transcript_ai_id),
    )
    .await?;
    repository::create_transcript_segments(&mut *transaction, id, &transcripts).await?;

    transaction.commit().await?;

    todo!()
}

fn generate_key(id: Uuid) -> String {
    format!("{}/audio", id)
}
