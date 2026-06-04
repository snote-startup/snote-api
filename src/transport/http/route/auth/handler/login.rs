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
#[schema(as = auth::login::Request)]
pub struct Request {
    #[schema(example = "user@example.com")]
    pub email: String,

    #[schema(example = "password123")]
    pub password: String,
}

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    post,
    operation_id = "auth::login",
    tag = "Auth",
    path = "/auth/login",
    request_body(
        content = Request,
        description = "Login using email and password"
    ),
    responses(
        (
            status = 201,
            description = "Login successful. Returns access token and sets refresh token cookie.",
            body = String,
            headers(
                ("Set-Cookie" = String),
            ),
            example = json!("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."),
        ),
        (
            status = 400,
            description = "Invalid email or password",
            body = ApiError
        ),
        (
            status = 500,
            description = "Internal server error",
            body = ApiError
        )
    )
)]
pub async fn login(
    State(state): State<Arc<ApiState>>,
    jar: CookieJar,
    Json(request): Json<Request>,
) -> ApiResult<(StatusCode, CookieJar, String)> {
    let token_pair = auth::service::login(
        &state.database,
        &state.token_util,
        &request.email,
        &request.password,
    )
    .await
    .with_context(StatusCode::BAD_REQUEST, "Invalid email or password")?;

    let mut cookie = Cookie::new(REFRESH_COOKIE, token_pair.refresh);
    cookie.set_same_site(SameSite::None);
    cookie.set_path(AUTH_ENDPOINT);

    Ok((StatusCode::CREATED, jar.add(cookie), token_pair.access))
}
