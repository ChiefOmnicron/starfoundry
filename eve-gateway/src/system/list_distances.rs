use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::system::SystemDistanceMinimal;
use crate::system::error::{Result, SystemError};

/// Fetch Distances
///
/// - Alternative route: `/latest/systems/distances`
/// - Alternative route: `/v1/systems/distances`
///
/// ---
///
/// Lists all system distances
#[utoipa::path(
    get,
    path = "/distances",
    tag = "System",
    responses(
        (
            body = Vec<SystemDistanceMinimal>,
            description = "Returns a list of all systems with their ranges",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let entry = list_distances(&state.postgres).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

async fn list_distances(pool: &PgPool) -> Result<Vec<SystemDistanceMinimal>> {
    let systems = sqlx::query!(
        r#"
            SELECT
                system_start,
                system_end,
                distance_ly
            FROM system_distance
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(SystemError::ListSystemDistance)?
    .into_iter()
    .map(|x| {
        SystemDistanceMinimal {
            system_start: x
                .system_start
                .into(),
            system_end:   x
                .system_end
                .into(),
            distance_ly:  x
                .distance_ly
                .into(),
        }
    })
    .collect::<Vec<_>>();

    Ok(systems)
}
