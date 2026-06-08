use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{
    error::{Error, Result},
    feature::auth::model::MinimalAccount,
    state::ApiState,
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "auth::me",
    tag = "Auth",
    path = "/auth/me",
    security(("jwt_token" = [])),
    responses(
        (
            status = 200,
            description = "Current authenticated account",
            body = MinimalAccount
        ),
        (
            status = 401,
            description = "Invalid, expired, or missing access token",
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn me(
    State(state): State<Arc<ApiState>>,
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<MinimalAccount>> {
    let access_token = bearer.token();
    state
        .auth_service
        .me(&state.db, &state.token_service, access_token)
        .await
        .map(Json)
}
