use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

/// Healthz
///
/// Checks if the service is ready to accept connections
/// 
pub async fn healthz() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(
            header::CACHE_CONTROL, "no-cache"
        )],
        "healthy"
    )
}
