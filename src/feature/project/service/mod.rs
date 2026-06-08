mod chat;
mod transcript;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::Result, feature::project::model::Project, state::ApiState};

pub use chat::*;
pub use transcript::*;

#[tracing::instrument(err(Debug), skip(database))]
pub async fn create(
    ApiState { db, .. }: &ApiState,

    account_id: Uuid,
    title: &str,
    description: Option<&str>,
) -> Result<Uuid> {
    todo!()
}

#[tracing::instrument(err(Debug), skip(database))]
pub async fn get_by_account(
    ApiState { db, .. }: &ApiState,

    account_id: Uuid,
) -> Result<Vec<Project>> {
    todo!()
}

#[tracing::instrument(err(Debug), skip(database))]
pub async fn get(
    ApiState { db, .. }: &ApiState,

    account_id: Uuid,
    id: Uuid,
) -> Result<Option<Project>> {
    todo!()
}

#[tracing::instrument(err(Debug), skip(database))]
pub async fn update(
    ApiState { db, .. }: &ApiState,

    account_id: Uuid,
    id: Uuid,
    title: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    todo!()
}
