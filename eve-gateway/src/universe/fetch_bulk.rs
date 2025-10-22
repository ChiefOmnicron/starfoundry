use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::SystemId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::universe::error::{Result, UniverseError};
use crate::universe::fetch::System;

/// Fetch an item
/// 
/// - Alternative route: `/latest/universe/systems/bulk`
/// - Alternative route: `/v1/universe/systems/bulk`
/// 
/// ---
/// 
/// Resolves all information about an item
/// 
#[utoipa::path(
    post,
    path = "/systems/bulk",
    tag = "Universe",
    request_body = Vec<SystemId>,
    responses(
        (
            body = Vec<System>,
            description = "Information about the given systems",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):     State<AppState>,
    Json(system_ids): Json<Vec<SystemId>>,
) -> Result<impl IntoResponse> {
    let entry = fetch_system_bulk(
        &state.postgres,
        system_ids,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}

pub async fn fetch_system_bulk(
    pool:       &PgPool,
    system_ids: Vec<SystemId>,
) -> Result<Vec<System>> {
    if system_ids.is_empty() {
        return Ok(Vec::new());
    }

    let systems = sqlx::query!("
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
            WHERE system_id = ANY($1)
        ",
            &system_ids.clone().into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(UniverseError::FetchSystemBulk)?
        .into_iter()
        .map(|x| System {
            region_id:          x.region_id.into(),
            region_name:        x.region_name,
            constellation_id:   x.constellation_id.into(),
            constellation_name: x.constellation_name,
            system_id:          x.system_id.into(),
            system_name:        x.system_name,
            security:           x.security,
            security_str:       x.security_str,
        })
        .collect::<Vec<_>>();

    Ok(
        systems
    )
}
