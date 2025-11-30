use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::AppState;
use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn assert_exists(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    request:                  Request,
    next:                     Next,
) -> Result<impl IntoResponse> {
    assert_exists_check(
            &state.pool,
            project_group_uuid,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_exists_check(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
) -> Result<()> {
    let project = sqlx::query!("
            SELECT id
            FROM project_group
            WHERE id = $1
        ",
            *project_group_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchGroup(e, project_group_uuid))?;

    if project.is_some() {
        Ok(())
    } else {
        Err(ProjectGroupError::NotFound(project_group_uuid))
    }
}
