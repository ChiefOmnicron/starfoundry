use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::AppState;
use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;

pub async fn assert_exists(
    State(state):     State<AppState>,
    Path(project_id): Path<ProjectUuid>,
    request:          Request,
    next:             Next,
) -> Result<impl IntoResponse> {
    assert_exists_check(
            &state.pool,
            project_id,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_exists_check(
    pool:       &PgPool,
    project_id: ProjectUuid,
) -> Result<()> {
    let project = sqlx::query!("
            SELECT id
            FROM project
            WHERE id = $1
        ",
            *project_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectError::FetchProject(e, project_id))?;

    if project.is_some() {
        Ok(())
    } else {
        Err(ProjectError::NotFound(project_id))
    }
}
