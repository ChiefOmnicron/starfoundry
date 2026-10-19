use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project::error::{ProjectError, Result};
use crate::{AppState, MarketUuid, ProjectUuid};

/// Delete market entry
///
/// - Alternative route: `/latest/projects/{ProjectUuid}/market/{MarketUuid}`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market/{MarketUuid}`
///
/// ---
///
/// Deletes a market entry
///
/// ## Security
/// - authenticated
#[utoipa::path(
    delete,
    path = "/{ProjectUuid}/market/{MarketUuid}",
    tag = "Project",
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "Market entry was deleted",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path((project_id, market_id)): Path<(ProjectUuid, MarketUuid)>,
) -> Result<impl IntoResponse> {
    delete_market_entry(&state.postgres, project_id, market_id).await?;

    Ok((StatusCode::NO_CONTENT, Json(())).into_response())
}

pub async fn delete_market_entry(
    pool: &PgPool,
    project_id: ProjectUuid,
    market_id: MarketUuid,
) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM project_market
            WHERE project_id = $1
            AND id = $2
        "#,
        *project_id,
        *market_id,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| ProjectError::Delete(e, project_id))
}
