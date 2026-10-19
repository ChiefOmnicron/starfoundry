use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project::error::{ProjectError, Result};
use crate::{AppState, ProjectUuid};

/// Delete project
///
/// - Alternative route: `/latest/projects/{ProjectUuid}`
/// - Alternative route: `/v1/projects/{ProjectUuid}`
///
/// ---
///
/// Deletes a project
///
/// ## Security
/// - authenticated
/// - structure:read
#[utoipa::path(
    delete,
    path = "/{ProjectUuid}",
    tag = "Project",
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "Project was deleted",
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
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    delete(&state.postgres, project_id).await?;

    Ok((StatusCode::NO_CONTENT, Json(())).into_response())
}

pub async fn delete(
    pool: &PgPool,
    project_id: ProjectUuid,
) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM project
            WHERE id = $1
        "#,
        *project_id,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| ProjectError::Delete(e, project_id))
}
