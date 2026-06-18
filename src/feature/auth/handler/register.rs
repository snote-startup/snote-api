use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    error::{Error, Result},
    feature::auth::{AUTH_ENDPOINT, REFRESH_COOKIE},
    shared::ApiState,
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

#[tracing::instrument(err(Debug), skip(state))]
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
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn register(
    State(state): State<Arc<ApiState>>,
    jar: CookieJar,
    Json(request): Json<Request>,
) -> Result<(StatusCode, CookieJar, String)> {
    let token_pair = state
        .auth_svc
        .register(
            &state.db,
            &state.token_svc,
            &request.email,
            &request.password,
            &request.name,
        )
        .await?;

    let mut cookie = Cookie::new(REFRESH_COOKIE, token_pair.refresh);
    cookie.set_same_site(SameSite::None);
    cookie.set_path(AUTH_ENDPOINT);

    Ok((StatusCode::CREATED, jar.add(cookie), token_pair.access))
}
