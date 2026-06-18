use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::Result, feature::project::service::ProjectService};

use super::{
    model::{Task, TaskPriority, TaskStatus},
    repository,
};

pub struct TaskService;

impl TaskService {
    pub async fn create(
        &self,

        db: &PgPool,
        project_svc: &ProjectService,

        project_id: Uuid,
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
