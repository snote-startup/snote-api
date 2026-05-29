use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    feature::auth,
    transport::http::{
        error::{ApiError, ApiResult, ResultExt},
        route::auth::{AUTH_ENDPOINT, REFRESH_COOKIE},
        state::ApiState,
    },
};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(as = auth::register::Request)]
pub struct Request {
    #[schema(example = "user@example.com")]
    pub email: String,

    #[schema(example = "password123")]
    pub password: String,

    #[schema(example = "John Doe")]
    pub name: String,
}

#[utoipa::path(
    post,
    operation_id = "auth::register",
    tag = "Auth",
    path = "/auth/register",
    request_body(
        content = Request,
        description = "Create a new account"
    ),
    responses(
        (
            status = 201,
            description = "Account created successfully. Returns access token and sets refresh token cookie.",
            body = String,
            headers(
                (
                    "Set-Cookie" = String,
                    description = "Refresh token cookie"
                )
            ),
            example = json!("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."),
        ),
        (
            status = 400,
            description = "Failed to create account",
            body = ApiError
        ),
        (
            status = 500,
            description = "Internal server error",
            body = ApiError
        )
    )
)]
pub async fn register(
    State(state): State<Arc<ApiState>>,
    jar: CookieJar,
    Json(request): Json<Request>,
) -> ApiResult<(StatusCode, CookieJar, String)> {
    let token_pair = auth::service::register(
        &state.database,
        &state.token_util,
        &request.email,
        &request.password,
        &request.name,
    )
    .await
    .with_context(StatusCode::BAD_REQUEST, "Failed to create account")?;

    let mut cookie = Cookie::new(REFRESH_COOKIE, token_pair.refresh);
    cookie.set_same_site(SameSite::None);
    cookie.set_path(AUTH_ENDPOINT);

    Ok((StatusCode::CREATED, jar.add(cookie), token_pair.access))
}
