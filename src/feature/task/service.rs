use rig_core::{client::CompletionClient, extractor::Extractor, providers::gemini};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::Result,
    feature::{
        project::service::ProjectService,
        task::{SYSTEM_PROMPT, model::CreateTaskData},
    },
};

use super::{
    model::{Task, TaskPriority, TaskStatus},
    repository,
};

pub struct TaskService {
    pub extractor: Extractor<gemini::CompletionModel, CreateTaskData>,
}

impl TaskService {
    #[tracing::instrument(err(Debug))]
    pub fn new(api_key: &str) -> color_eyre::Result<Self> {
        let client = gemini::Client::new(api_key)?;

        let extractor = client
            .extractor::<CreateTaskData>(gemini::completion::GEMINI_3_FLASH_PREVIEW)
            .preamble(SYSTEM_PROMPT)
            .build();

        Ok(Self { extractor })
    }

    pub async fn create(
        &self,

        _db: &PgPool,
        _project_svc: &ProjectService,

        _project_id: Uuid,
    ) -> Result<()> {
        todo!()
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
