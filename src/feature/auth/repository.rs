use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::Account;

pub async fn create_account(
    executor: impl PgExecutor<'_>,
    email: &str,
    password: &str,
    name: &str,
) -> sqlx::Result<Uuid> {
    sqlx::query_scalar!(
        r#"
            INSERT INTO accounts(email, password, name)
            VALUES($1, $2, $3)
            RETURNING id;
        "#,
        email,
        password,
        name
    )
    .fetch_one(executor)
    .await
}

pub async fn get_account(executor: impl PgExecutor<'_>, id: Uuid) -> sqlx::Result<Account> {
    sqlx::query_as!(
        Account,
        r#"
            SELECT id, email, password, name, role as "role: _", is_active
            FROM accounts
            WHERE id = $1
        "#,
        id
    )
    .fetch_one(executor)
    .await
}

pub async fn get_account_by_email(
    executor: impl PgExecutor<'_>,
    email: &str,
) -> sqlx::Result<Account> {
    sqlx::query_as!(
        Account,
        r#"
            SELECT id, email, password, name, role as "role: _", is_active
            FROM accounts
            WHERE email = $1
        "#,
        email
    )
    .fetch_one(executor)
    .await
}
