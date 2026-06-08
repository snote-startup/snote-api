use std::sync::Arc;

use axum::{RequestPartsExt, extract::FromRequestParts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    error::{Error, ResultExt},
    state::ApiState,
};

pub struct AccountID(pub Uuid);

impl FromRequestParts<Arc<ApiState>> for AccountID {
    type Rejection = Error;

    #[tracing::instrument(err(Debug), skip(state))]
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &Arc<ApiState>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .with_context(StatusCode::UNAUTHORIZED, "Missing access token")?;
        let access_token = bearer.token();
        let id = state
            .token_service
            .access
            .decode(access_token)
            .with_context(StatusCode::UNAUTHORIZED, "Invalid token")?;
        Ok(AccountID(id))
    }
}
