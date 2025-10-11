use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

use crate::state::AppState;

/// Readyz
/// 
/// Checks that the database connection is up and running
/// 
pub async fn readyz(
    State(state): State<AppState>,
) -> impl IntoResponse {
    if state.routes.is_empty() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(
                header::CACHE_CONTROL, "no-cache"
            )],
            "unhealthy"
        );
    }

    (
        StatusCode::OK,
        [(
            header::CACHE_CONTROL, "no-cache"
        )],
        "healthy"
    )
}
