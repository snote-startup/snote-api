#[utoipa::path(
    get,
    tag = "Health",
    path = "/health",
    responses(
        (
            status = 200,
            body = String,
        ),
    )
)]
pub async fn health() -> &'static str {
    "ok"
}
