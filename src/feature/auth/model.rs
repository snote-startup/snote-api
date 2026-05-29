use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, sqlx::Type, ToSchema)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Member,
}

pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: Role,
}

#[derive(Serialize, ToSchema)]
pub struct MinimalAccount {
    pub email: String,
    pub name: String,
    pub role: Role,
}

pub struct TokenPair {
    pub access: String,
    pub refresh: String,
}
