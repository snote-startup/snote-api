use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{
    feature::auth::{self, model::MinimalAccount},
    transport::http::{
        error::{ApiError, ApiResult, ResultExt},
        state::ApiState,
    },
};

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
            body = ApiError
        ),
        (
            status = 500,
            description = "Internal server error",
            body = ApiError
        )
    )
)]
pub async fn me(
    State(state): State<Arc<ApiState>>,
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
) -> ApiResult<Json<MinimalAccount>> {
    let access_token = bearer.token();
    auth::service::me(&state.database, &state.token_util, access_token)
        .await
        .map(Json)
        .with_context(StatusCode::UNAUTHORIZED, "Invalid access token")
}
