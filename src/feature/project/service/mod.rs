mod chat;
mod transcript;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::Result, feature::project::model::Project, state::AppState};

pub use chat::*;
pub use transcript::*;

#[tracing::instrument(err(Debug), skip(database))]
pub async fn create(
    AppState { db, .. }: &AppState,

    account_id: Uuid,
    title: &str,
    description: Option<&str>,
) -> Result<Uuid> {
    todo!()
}

#[tracing::instrument(err(Debug), skip(database))]
pub async fn get_by_account(
    AppState { db, .. }: &AppState,

    account_id: Uuid,
) -> Result<Vec<Project>> {
    todo!()
}

#[tracing::instrument(err(Debug), skip(database))]
pub async fn get(
    AppState { db, .. }: &AppState,

    account_id: Uuid,
    id: Uuid,
) -> Result<Option<Project>> {
    todo!()
}

#[tracing::instrument(err(Debug), skip(database))]
pub async fn update(
    AppState { db, .. }: &AppState,

    account_id: Uuid,
    id: Uuid,
    title: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    todo!()
}
