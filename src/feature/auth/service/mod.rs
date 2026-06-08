mod token;

use http::StatusCode;
use sqlx::PgPool;

use crate::{
    error::{Error, ErrorContext, Result, ResultExt},
    feature::auth::{
        model::{MinimalAccount, TokenPair},
        repository,
    },
};

pub use token::*;

const BCRYPT_COST: u32 = 10;

pub struct AuthService;

impl AuthService {
    #[tracing::instrument(err(Debug), skip(self, db, token_service))]
    pub async fn register(
        &self,

        db: &PgPool,
        token_service: &TokenService,

        email: &str,
        password: &str,
        name: &str,
    ) -> Result<TokenPair> {
        let hashed_password = bcrypt::hash(password, BCRYPT_COST)?;
        let id = repository::create_account(db, email, &hashed_password, name).await?;

        token_service.encode(id)
    }

    #[tracing::instrument(err(Debug), skip(self, db, token_service))]
    pub async fn login(
        &self,

        db: &PgPool,
        token_service: &TokenService,

        email: &str,
        password: &str,
    ) -> Result<TokenPair> {
        let account = repository::get_account_by_email(db, email).await?;
        if !bcrypt::verify(password, &account.password)? {
            return Err(ErrorContext {
                status: StatusCode::BAD_REQUEST,
                message: "Wrong password".to_string(),
                ..Default::default()
            }
            .into());
        }
        let id = account.id;

        token_service.encode(id)
    }

    #[tracing::instrument(err(Debug), skip(self, db, token_service))]
    pub async fn me(
        &self,

        db: &PgPool,
        token_service: &TokenService,

        access_token: &str,
    ) -> Result<MinimalAccount> {
        let id = token_service.access.decode(access_token)?;
        let account = repository::get_account(db, id).await?;

        Ok(MinimalAccount {
            email: account.email,
            name: account.name,
            role: account.role,
        })
    }

    #[tracing::instrument(err(Debug), skip(self, token_service))]
    pub fn refresh(&self, token_service: &TokenService, refresh_token: &str) -> Result<TokenPair> {
        let id = token_service
            .refresh
            .decode(refresh_token)
            .with_context(StatusCode::UNAUTHORIZED, "Invalid refresh token")?;

        token_service.encode(id)
    }
}
