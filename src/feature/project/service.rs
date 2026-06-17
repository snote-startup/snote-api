use aws_sdk_s3::primitives::ByteStream;
use http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{ErrorContext, Result},
    feature::project::{
        model::{Project, TranscriptSegment},
        repository,
    },
    infra::{storage::S3Client, transcript::AssemblyAIClient},
};

#[derive(Clone, Copy)]
pub struct ProjectService;

impl ProjectService {
    #[tracing::instrument(err(Debug), skip(self, db))]
    pub async fn create(
        &self,

        db: &PgPool,

        account_id: Uuid,
        title: &str,
        description: Option<&str>,
    ) -> Result<Uuid> {
        let id = repository::create_project(db, account_id, title, description).await?;
        Ok(id)
    }

    #[tracing::instrument(err(Debug), skip(self, db))]
    pub async fn get_by_account(&self, db: &PgPool, account_id: Uuid) -> Result<Vec<Project>> {
        let projects = repository::get_projects_by_account(db, account_id).await?;
        Ok(projects)
    }

    #[tracing::instrument(err(Debug), skip(self, db))]
    pub async fn get(&self, db: &PgPool, account_id: Uuid, id: Uuid) -> Result<Project> {
        match repository::get_project(db, account_id, id).await? {
            Some(project) => Ok(project),
            None => Err(ErrorContext {
                status: StatusCode::NOT_FOUND,
                message: "No project with given id".to_string(),
                ..Default::default()
            }
            .into()),
        }
    }

    #[tracing::instrument(err(Debug), skip(self, db))]
    pub async fn update(
        &self,

        db: &PgPool,

        account_id: Uuid,
        id: Uuid,
        title: Option<&str>,
        description: Option<&str>,
    ) -> Result<()> {
        let _ = self.get(db, account_id, id).await?;
        repository::update_project(db, id, title, description, None, None).await?;
        Ok(())
    }

    #[tracing::instrument(err(Debug), skip(self, db, s3_client))]
    pub async fn upload_audio(
        &self,

        db: &PgPool,
        s3_client: &S3Client,

        account_id: Uuid,
        id: Uuid,
        content: ByteStream,
    ) -> Result<()> {
        let _ = self.get(db, account_id, id).await?;

        let key = format!("{}/audio", id);
        let audio_url = s3_client.upload(key, content).await?;

        repository::update_project(db, id, None, None, Some(&audio_url), None).await?;

        Ok(())
    }

    #[tracing::instrument(err(Debug), skip(self, db, assembly_ai_client))]
    pub async fn create_transcript(
        &self,

        db: &PgPool,
        assembly_ai_client: &AssemblyAIClient,

        account_id: Uuid,
        id: Uuid,
    ) -> Result<()> {
        let project = self.get(db, account_id, id).await?;
        let Some(audio_url) = project.audio_url.clone() else {
            return Err(ErrorContext {
                status: StatusCode::NOT_FOUND,
                message: "No audio file found in project".into(),
                ..Default::default()
            }
            .into());
        };

        let transcript_ai_id = assembly_ai_client.create_transcript(&audio_url).await?;
        let transcript = assembly_ai_client.get_transcript(&transcript_ai_id).await?;

        let speakers: Vec<_> = transcript.iter().map(|x| x.speaker.clone()).collect();
        let texts: Vec<_> = transcript.iter().map(|x| x.text.clone()).collect();
        let starts: Vec<_> = transcript.iter().map(|x| x.start).collect();
        let ends: Vec<_> = transcript.iter().map(|x| x.end).collect();

        let mut transaction = db.begin().await?;

        repository::update_project(
            &mut *transaction,
            id,
            None,
            None,
            None,
            Some(&transcript_ai_id),
        )
        .await?;
        repository::create_transcript_segments(
            &mut *transaction,
            id,
            &speakers,
            &texts,
            &starts,
            &ends,
        )
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[tracing::instrument(err(Debug), skip(self, db))]
    pub async fn get_transcript(
        &self,

        db: &PgPool,

        account_id: Uuid,
        id: Uuid,
    ) -> Result<Vec<TranscriptSegment>> {
        let transcripts = repository::get_transcript_segments(db, id).await?;

        Ok(transcripts)
    }
}
