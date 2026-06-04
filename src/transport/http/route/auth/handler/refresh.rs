use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use crate::{
    feature::auth,
    transport::http::{
        error::{ApiError, ApiResult, Context, ResultExt as _},
        route::auth::{AUTH_ENDPOINT, REFRESH_COOKIE},
        state::ApiState,
    },
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
            body = ApiError
        ),
        (
            status = 500,
            description = "Internal server error",
            body = ApiError
        )
    )
)]
pub async fn refresh(
    State(state): State<Arc<ApiState>>,
    jar: CookieJar,
) -> ApiResult<(CookieJar, String)> {
    let Some(cookie) = jar.get(REFRESH_COOKIE).cloned() else {
        return Err(Context {
            status: StatusCode::UNAUTHORIZED,
            message: "Missing refresh token".to_string(),
            ..Default::default()
        }
        .into());
    };
    let refresh_token = cookie.value();
    let token_pair = auth::service::refresh(&state.token_util, refresh_token)
        .with_context(StatusCode::UNAUTHORIZED, "Invalid refresh token")?;

    let mut cookie = Cookie::new(REFRESH_COOKIE, token_pair.refresh);
    cookie.set_same_site(SameSite::None);
    cookie.set_path(AUTH_ENDPOINT);

    Ok((jar.add(cookie), token_pair.access))
}
