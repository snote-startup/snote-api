use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::Result,
    feature::{
        project::service::ProjectService,
        task::model::{TaskPriority, TaskStatus},
    },
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
        todo!()
    }

    pub async fn update(
        &self,

        db: &PgPool,

        id: Uuid,
        status: Option<TaskStatus>,
        priority: Option<TaskPriority>,
        content: Option<&str>,
    ) -> Result<()> {
        todo!()
    }

    pub async fn delete(&self, db: &PgPool, id: Uuid) -> Result<()> {
        todo!()
    }
}
