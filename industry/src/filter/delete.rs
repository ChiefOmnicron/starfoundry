use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::FilterUuid;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::filter::error::{FilterError, Result};

/// Delete Filter
/// 
/// - Alternative route: `/latest/filters/{FilterUuid}`
/// - Alternative route: `/v1/filters/{FilterUuid}`
/// 
/// ---
/// 
/// Deletes a filter
/// 
/// ## Security
/// - authenticated
/// - owner
/// 
#[utoipa::path(
    delete,
    path = "/{FilterUuid}",
    tag = "filters",
    responses(
        (
            description = "The filter was deleted",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Path(filter_id):    Path<FilterUuid>,
) -> Result<impl IntoResponse> {
    let result = delete(
            &state.postgres,
            identity.character_id,
            filter_id,
        )
        .await?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(result)
        )
    )
}

async fn delete(
    pool:           &PgPool,
    character_id:   CharacterId,
    filter_id:      FilterUuid,
) -> Result<()> {
    let result = sqlx::query!("
            DELETE FROM filter
            WHERE id = $1
            AND owner = $2
        ",
            *filter_id,
            *character_id,
        )
        .execute(pool)
        .await
        .map_err(FilterError::Delete)?;

    if result.rows_affected() == 0 {
        Err(FilterError::NotFound(filter_id))
    } else {
        Ok(())
    }
}
