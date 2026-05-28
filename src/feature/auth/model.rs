use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Member,
}

pub struct Account {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: Role,
    pub is_active: bool,
}

pub struct MinimalAccount {
    pub email: String,
    pub name: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: u64,
}
