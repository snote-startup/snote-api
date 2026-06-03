use aws_sdk_s3::primitives::ByteStream;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{feature::project::repository, util::storage};

pub async fn upload_audio(
    database: &PgPool,
    s3: &aws_sdk_s3::Client,
    id: Uuid,
    account_id: Uuid,
    content: ByteStream,
) -> color_eyre::Result<()> {
    let key = generate_key(id);
    let audio_url = storage::upload(s3, key, content).await?;

    repository::update_project(database, id, account_id, None, None, Some(&audio_url), None)
        .await?;

    todo!()
}

fn generate_key(id: Uuid) -> String {
    format!("{}/audio", id)
}
