use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::system::error::{Result, SystemError};
use crate::system::{System, SystemSearchQuery};

/// List Systems
///
/// - Alternative route: `/latest/systems`
/// - Alternative route: `/v1/systems`
///
/// ---
///
/// Resolves all information about a system
#[utoipa::path(
    get,
    path = "/",
    tag = "System",
    params(
        ("name" = String, Query),
    ),
    responses(
        (
            body = Vec<System>,
            description = "Returns a list of all systems",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Query(filter): Query<SystemSearchQuery>,
) -> Result<impl IntoResponse> {
    let entry = list(&state.postgres, filter).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

async fn list(
    pool: &PgPool,
    filter: SystemSearchQuery,
) -> Result<Vec<System>> {
    let systems = sqlx::query!(
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
            WHERE
            (
                NOT (LOWER(system_name) LIKE '%' || LOWER($1) || '%') IS FALSE
            )
            ORDER BY system_name ASC
        ",
        filter.name,
    )
    .fetch_all(pool)
    .await
    .map_err(SystemError::ListSystem)?
    .into_iter()
    .map(|x| {
        System {
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
        }
    })
    .collect::<Vec<_>>();

    Ok(systems)
}
