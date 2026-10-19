use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use chrono::{Days, NaiveTime};
use reqwest::StatusCode;
use sqlx::PgPool;
use starfoundry_lib_types::SystemId;

use crate::TimeSpanQuery;
use crate::api_docs::{InternalServerError, NotFound};
use crate::industry::error::{IndustryError, Result};
use crate::industry::models::{SystemIndex, SystemIndexHistory};
use crate::state::AppState;
use crate::system::fetch_system;

/// List System-Index History
///
/// - Alternative route: `/latest/industry/system-index/{SystemId}/history`
/// - Alternative route: `/v1/industry/system-index/{SystemId}/history`
///
/// ---
///
/// Loads all open orders from a character
#[utoipa::path(
    get,
    path = "/system-index/{SystemId}/history",
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
    Query(time_span): Query<TimeSpanQuery>,
) -> Result<impl IntoResponse> {
    let system_index =
        list_system_index(&state.postgres, system_id, time_span).await?;

    if system_index.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(system_index)).into_response())
    } else {
        Ok((StatusCode::OK, Json(system_index)).into_response())
    }
}

async fn list_system_index(
    pool: &PgPool,
    system_id: SystemId,
    time_span: TimeSpanQuery,
) -> Result<Vec<SystemIndexHistory>> {
    let index = sqlx::query!(
        r#"
            SELECT
                timestamp,
                manufacturing,
                reaction,
                copying,
                invention,
                research_time,
                research_material
            FROM system_index
            WHERE system_id = $1
            AND timestamp >= $2
            AND timestamp <= $3
            ORDER BY timestamp DESC
        "#,
        *system_id,
        time_span
            .start
            .and_time(NaiveTime::default()),
        time_span
            .end
            .and_time(NaiveTime::default())
            .checked_add_days(Days::new(1)),
    )
    .fetch_all(pool)
    .await
    .map_err(|e| IndustryError::ListSystemIndex(e, system_id))?;

    let system = fetch_system(pool, system_id)
        .await?
        .ok_or(IndustryError::NoSystem)?;

    let index = index
        .into_iter()
        .map(|x| {
            SystemIndexHistory {
                system:               system.clone(),
                copying:              x.copying,
                invention:            x.invention,
                manufacturing:        x.manufacturing,
                reaction:             x.reaction,
                researching_material: x.research_material,
                researching_time:     x.research_time,
                timestamp:            x.timestamp,
            }
        })
        .collect::<Vec<_>>();
    Ok(index)
}
