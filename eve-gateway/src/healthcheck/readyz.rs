use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

use crate::state::AppState;
use starfoundry_lib_eve_client::EveApiClient;

/// Readyz
/// 
/// Checks that the database connection is up and running
/// 
#[utoipa::path(
    get,
    path = "/readyz",
    tag = "healthcheck",
    responses(
        (
            body = String,
            description = "Everything is a-okay",
            status = OK,
            example = json!("healthy"),
        ),
        (
            body = String,
            description = "Not ready",
            status = INTERNAL_SERVER_ERROR,
            example = json!("unhealthy"),
        )
    ),
)]
pub async fn readyz(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let pool = state.postgres.clone();

    let postgres_version = sqlx::query!("SELECT version()")
        .fetch_one(&pool)
        .await;

    if postgres_version.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(
                header::CACHE_CONTROL, "no-cache"
            )],
            "db unhealthy"
        );
    }

    let api_client = match EveApiClient::new(state.eve_api_metric) {
        Ok(x) => x,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CACHE_CONTROL, "no-cache"
                )],
                "EVE-Api client unhealthy"
            );
        }
    };
    if api_client
        .fetch_page::<serde_json::Value>("/status")
        .await
        .is_err() {

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(
                header::CACHE_CONTROL, "no-cache"
            )],
            "EVE-Api unhealthy"
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
