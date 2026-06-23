use rig_core::{client::CompletionClient, extractor::Extractor, providers::gemini};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::Result,
    feature::{
        project::service::ProjectService,
        task::{SYSTEM_PROMPT, model::LLMResponse},
    },
};

use super::{
    model::{Task, TaskPriority, TaskStatus},
    repository,
};

pub struct TaskService {
    pub extractor: Extractor<gemini::CompletionModel, LLMResponse>,
}

impl TaskService {
    #[tracing::instrument(err(Debug))]
    pub fn new(api_key: &str) -> color_eyre::Result<Self> {
        let client = gemini::Client::new(api_key)?;

        let extractor = client
            .extractor::<LLMResponse>(gemini::completion::GEMINI_3_FLASH_PREVIEW)
            .preamble(SYSTEM_PROMPT)
            .build();

        Ok(Self { extractor })
    }

    pub async fn create(
        &self,

        db: &PgPool,
        project_svc: &ProjectService,

        account_id: Uuid,
        project_id: Uuid,
    ) -> Result<()> {
        let transcript = project_svc
            .get_transcript(db, account_id, project_id)
            .await?;
        let mut prompt = String::new();
        for segment in transcript {
            prompt.push_str(&segment.to_string());
            prompt.push('\n');
        }

        let resp = self.extractor.extract(prompt).await?;

        repository::create_tasks(db, project_id, resp.task).await?;

        Ok(())
    }

    pub async fn get_by_project(&self, db: &PgPool, project_id: Uuid) -> Result<Vec<Task>> {
        let tasks = repository::get_tasks_by_project(db, project_id).await?;

        Ok(tasks)
    }

    pub async fn update(
        &self,

        db: &PgPool,

        id: Uuid,
        status: Option<TaskStatus>,
        priority: Option<TaskPriority>,
        content: Option<&str>,
    ) -> Result<()> {
        repository::update_task(db, id, status, priority, content).await?;

        Ok(())
    }

    pub async fn delete(&self, db: &PgPool, id: Uuid) -> Result<()> {
        repository::delete_task(db, id).await?;

        Ok(())
    }
}
