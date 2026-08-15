use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

/// Healthz
///
/// Checks if the service is ready to accept connections
/// 
#[utoipa::path(
    get,
    path = "/healthz",
    tag = "healthcheck",
    responses(
        (
            body = String,
            description = "Everything is a-okay",
            status = OK,
            example = json!("healthy"),
        ),
    ),
)]
pub async fn healthz() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(
            header::CACHE_CONTROL, "no-cache"
        )],
        "healthy"
    )
}
