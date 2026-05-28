use uuid::Uuid;

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

pub struct MinimalAccount {
    pub email: String,
    pub name: String,
    pub role: Role,
}
