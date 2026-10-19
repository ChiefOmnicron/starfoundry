use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::{SystemId, TypeId};

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::system::System;
use crate::system::error::{Result, SystemError};

/// Fetch System
///
/// - Alternative route: `/latest/systems/{SystemId}`
/// - Alternative route: `/v1/systems/{SystemId}`
///
/// ---
///
/// Resolves all information about a system
#[utoipa::path(
    get,
    path = "/{SystemId}",
    tag = "System",
    params(
        TypeId,
    ),
    responses(
        (
            body = System,
            description = "Information about a system",
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
    let entry = fetch_system(&state.postgres, system_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

pub(crate) async fn fetch_system(
    pool: &PgPool,
    system_id: SystemId,
) -> Result<Option<System>> {
    let system = sqlx::query!(
        "
            SELECT
                region_id,
                region_name,
                constellation_id,
                constellation_name,
                system_id,
                system_name,
                security,
                security_str
            FROM system
            WHERE system_id = $1
        ",
        *system_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| SystemError::FetchSystem(e, system_id))?;

    if let Some(x) = system {
        Ok(Some(System {
            region_id:          x
                .region_id
                .into(),
            region_name:        x.region_name,
            constellation_id:   x
                .constellation_id
                .into(),
            constellation_name: x.constellation_name,
            system_id:          x
                .system_id
                .into(),
            system_name:        x.system_name,
            security:           x.security,
            security_str:       x.security_str,
        }))
    } else {
        Ok(None)
    }
}
