use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use crate::{
    error::{Error, ErrorContext, Result},
    feature::auth::{AUTH_ENDPOINT, REFRESH_COOKIE},
    shared::ApiState,
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    post,
    operation_id = "auth::refresh",
    tag = "Auth",
    path = "/auth/refresh",
    responses(
        (
            status = 200,
            description = "Refresh successful. Returns a new access token and sets a new refresh token cookie.",
            body = String,
            headers(
                (
                    "Set-Cookie" = String,
                    description = "New refresh token cookie"
                )
            )
        ),
        (
            status = 401,
            description = "Missing or invalid refresh token",
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn refresh(
    State(state): State<Arc<ApiState>>,
    jar: CookieJar,
) -> Result<(CookieJar, String)> {
    let Some(cookie) = jar.get(REFRESH_COOKIE).cloned() else {
        return Err(ErrorContext {
            status: StatusCode::UNAUTHORIZED,
            message: "Missing refresh token".to_string(),
            ..Default::default()
        }
        .into());
    };
    let refresh_token = cookie.value();
    let token_pair = state.auth_svc.refresh(&state.token_svc, refresh_token)?;

    let mut cookie = Cookie::new(REFRESH_COOKIE, token_pair.refresh);
    cookie.set_same_site(SameSite::None);
    cookie.set_path(AUTH_ENDPOINT);

    Ok((jar.add(cookie), token_pair.access))
}
