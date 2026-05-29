use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
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
    pub is_active: bool,
}

pub struct AccountCredential {
    pub id: Uuid,
    pub password: String,
}

pub struct MinimalAccount {
    pub email: String,
    pub name: String,
    pub role: Role,
}

pub struct TokenPair {
    pub access: String,
    pub refresh: String,
}
