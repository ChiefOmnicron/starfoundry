use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project::error::{ProjectError, Result};
use crate::{AppState, ProjectJobUuid, ProjectUuid};

/// Delete project
///
/// - Alternative route: `/latest/projects/{ProjectUuid}/jobs/{ProjectJobUuid}`
/// - Alternative route: `/v1/projects/{ProjectUuid}/jobs/{ProjectJobUuid}`
///
/// ---
///
/// Fetches information about a structure
///
/// ## Security
/// - authenticated
/// - structure:read
#[utoipa::path(
    delete,
    path = "/{ProjectUuid}/jobs/{ProjectJobUuid}",
    tag = "Project",
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "Project job was deleted",
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
    Path((project_id, job_id)): Path<(ProjectUuid, ProjectJobUuid)>,
) -> Result<impl IntoResponse> {
    delete_job(&state.postgres, project_id, job_id).await?;

    Ok((StatusCode::NO_CONTENT, Json(())).into_response())
}

pub async fn delete_job(
    pool: &PgPool,
    project_id: ProjectUuid,
    job_id: ProjectJobUuid,
) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM project_job
            WHERE id = $1
            AND project_id = $2
        "#,
        *job_id,
        *project_id,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| ProjectError::Delete(e, project_id))
}
