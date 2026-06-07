use sqlx::PgPool;

use crate::{
    error::{Error, Result},
    feature::auth::{model::MinimalAccount, repository},
    shared::token::{model::TokenPair, service::TokenService},
};

const BCRYPT_COST: u32 = 10;

#[tracing::instrument(err(Debug), skip(database, token_service))]
pub async fn register(
    database: &PgPool,
    token_service: &TokenService,
    email: &str,
    password: &str,
    name: &str,
) -> Result<TokenPair> {
    let hashed_password =
        bcrypt::hash(password, BCRYPT_COST).map_err(color_eyre::eyre::Error::from)?;
    let id = repository::create_account(database, email, &hashed_password, name).await?;

    token_service.encode(id)
}

#[tracing::instrument(err(Debug), skip(database, token_service))]
pub async fn login(
    database: &PgPool,
    token_service: &TokenService,
    email: &str,
    password: &str,
) -> Result<TokenPair> {
    let account = repository::get_account_by_email(database, email).await?;
    if !bcrypt::verify(password, &account.password).map_err(color_eyre::eyre::Error::from)? {
        return Err(Error::WrongPassword);
    }
    let id = account.id;

    token_service.encode(id)
}

#[tracing::instrument(err(Debug), skip(database, token_service))]
pub async fn me(
    database: &PgPool,
    token_service: &TokenService,
    access_token: &str,
) -> Result<MinimalAccount> {
    let id = token_service.access.decode(access_token)?;
    let account = repository::get_account(database, id).await?;

    Ok(MinimalAccount {
        email: account.email,
        name: account.name,
        role: account.role,
    })
}

#[tracing::instrument(err(Debug), skip(token_service))]
pub fn refresh(token_service: &TokenService, refresh_token: &str) -> Result<TokenPair> {
    let id = token_service.refresh.decode(refresh_token)?;

    token_service.encode(id)
}
