use color_eyre::eyre::bail;

use sqlx::PgPool;

use crate::{
    config::CONFIG,
    feature::auth::{
        model::{MinimalAccount, TokenPair},
        repository,
    },
    util::token::CompletedTokenUtil,
};

#[tracing::instrument(err(Debug), skip(database, token_util))]
pub async fn register(
    database: &PgPool,
    token_util: &CompletedTokenUtil,
    email: &str,
    password: &str,
    name: &str,
) -> color_eyre::Result<TokenPair> {
    let hashed_password = bcrypt::hash(password, CONFIG.bcrypt_cost)?;
    let id = repository::create_account(database, email, &hashed_password, name).await?;

    let access_token = token_util.access.encode(id)?;
    let refresh_token = token_util.refresh.encode(id)?;

    Ok(TokenPair {
        access: access_token,
        refresh: refresh_token,
    })
}

#[tracing::instrument(err(Debug), skip(database, token_util))]
pub async fn login(
    database: &PgPool,
    token_util: &CompletedTokenUtil,
    email: &str,
    password: &str,
) -> color_eyre::Result<TokenPair> {
    let account = repository::get_account_by_email(database, email).await?;
    if !bcrypt::verify(password, &account.password)? {
        bail!("invalid password")
    }

    let id = account.id;
    let access_token = token_util.access.encode(id)?;
    let refresh_token = token_util.refresh.encode(id)?;

    Ok(TokenPair {
        access: access_token,
        refresh: refresh_token,
    })
}

#[tracing::instrument(err(Debug), skip(database, token_util))]
pub async fn me(
    database: &PgPool,
    token_util: &CompletedTokenUtil,
    access_token: &str,
) -> color_eyre::Result<MinimalAccount> {
    let id = token_util.access.decode(access_token)?;
    let account = repository::get_account(database, id).await?;

    Ok(MinimalAccount {
        email: account.email,
        name: account.name,
        role: account.role,
    })
}

#[tracing::instrument(err(Debug), skip(token_util))]
pub fn refresh(
    token_util: &CompletedTokenUtil,
    refresh_token: &str,
) -> color_eyre::Result<TokenPair> {
    let id = token_util.refresh.decode(refresh_token)?;

    Ok(TokenPair {
        refresh: token_util.refresh.encode(id)?,
        access: token_util.access.encode(id)?,
    })
}
