use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::{AppState, ProjectGroupUuid};

/// Delete Group
///
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}/archive`
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}/archive`
///
/// ---
///
/// Deletes the group
///
/// ## Security
/// - authenticated
/// - project_group:owner
#[utoipa::path(
    put,
    path = "/{ProjectGroupUuid}/archive",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            description = "The group was successfully archived",
            status = NO_CONTENT,
        ),
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    archive(&state.postgres, project_group_uuid).await?;

    Ok((StatusCode::NO_CONTENT, ()))
}

pub async fn archive(
    pool: &PgPool,
    group_id: ProjectGroupUuid,
) -> Result<()> {
    sqlx::query!(
        "
        UPDATE project_group
        SET archived = NOT archived
        WHERE id = $1
    ",
        *group_id,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| ProjectGroupError::ArchiveGroup(e, group_id))
}
