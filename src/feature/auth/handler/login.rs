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
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn login(
    State(state): State<Arc<ApiState>>,
    jar: CookieJar,
    Json(request): Json<Request>,
) -> Result<(StatusCode, CookieJar, String)> {
    let token_pair = state
        .auth_svc
        .login(
            &state.db,
            &state.token_svc,
            &request.email,
            &request.password,
        )
        .await?;

    let mut cookie = Cookie::new(REFRESH_COOKIE, token_pair.refresh);
    cookie.set_same_site(SameSite::None);
    cookie.set_path(AUTH_ENDPOINT);

    Ok((StatusCode::CREATED, jar.add(cookie), token_pair.access))
}
