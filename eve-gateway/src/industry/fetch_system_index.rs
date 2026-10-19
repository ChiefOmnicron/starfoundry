use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use reqwest::StatusCode;
use sqlx::PgPool;
use starfoundry_lib_types::SystemId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::industry::error::{IndustryError, Result};
use crate::industry::models::SystemIndex;
use crate::state::AppState;
use crate::system::fetch_system;

/// Fetch System-Index
///
/// - Alternative route: `/latest/industry/system-index/{SystemId}`
/// - Alternative route: `/v1/industry/system-index/{SystemId}`
///
/// ---
///
/// Loads all open orders from a character
#[utoipa::path(
    get,
    path = "/system-index/{SystemId}",
    tag = "Industry",
    responses(
        (
            body = SystemIndex,
            description = "Index for the system",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(system_id): Path<SystemId>,
) -> Result<impl IntoResponse> {
    let system_index = fetch_system_index(&state.postgres, system_id).await?;

    if system_index.is_none() {
        Ok((StatusCode::NO_CONTENT, Json(system_index)).into_response())
    } else {
        Ok((StatusCode::OK, Json(system_index)).into_response())
    }
}

async fn fetch_system_index(
    pool: &PgPool,
    system_id: SystemId,
) -> Result<Option<SystemIndex>> {
    let index = sqlx::query!(
        r#"
            SELECT
                manufacturing,
                reaction,
                copying,
                invention,
                research_time,
                research_material
            FROM system_index
            WHERE system_id = $1
            ORDER BY timestamp DESC
        "#,
        *system_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| IndustryError::FetchSystemIndex(e, system_id))?;

    if let Some(x) = index {
        let system = fetch_system(pool, system_id)
            .await?
            .ok_or(IndustryError::NoSystem)?;

        Ok(Some(SystemIndex {
            system:               system,
            copying:              x.copying,
            invention:            x.invention,
            manufacturing:        x.manufacturing,
            reaction:             x.reaction,
            researching_material: x.research_material,
            researching_time:     x.research_time,
        }))
    } else {
        Ok(None)
    }
}
