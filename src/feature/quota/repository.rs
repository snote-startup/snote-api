use sqlx::PgExecutor;
use uuid::Uuid;

pub async fn create_quota(executor: impl PgExecutor<'_>, account_id: Uuid) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO quotas(account_id)
            VALUES ($1)
            ON CONFLICT DO NOTHING
        "#,
        account_id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_quota(executor: impl PgExecutor<'_>, account_id: Uuid) -> sqlx::Result<i32> {
    sqlx::query_scalar!(
        r#"
            SELECT count
            FROM quotas
            WHERE account_id = $1
        "#,
        account_id
    )
    .fetch_one(executor)
    .await
}

pub async fn update_quota(
    executor: impl PgExecutor<'_>,
    account_id: Uuid,
    quota_diff: i32,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            UPDATE quotas
            SET count = count + $2
            WHERE account_id = $1
        "#,
        account_id,
        quota_diff
    )
    .execute(executor)
    .await?;

    Ok(())
}
