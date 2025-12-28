use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::AppState;
use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;

pub async fn assert_read(
    State(state):       State<AppState>,
    Path(project_uuid): Path<ProjectUuid>,
    identity:           ExtractIdentity,
    request:            Request,
    next:               Next,
) -> Result<impl IntoResponse> {
    assert_read_access_check(
            &state.pool,
            project_uuid,
            identity.character_id,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_read_access_check(
    pool:         &PgPool,
    project_id:   ProjectUuid,
    character_id: CharacterId,
) -> Result<()> {
    let project = sqlx::query!("
            SELECT project_group_id
            FROM project
            WHERE id = $1
        ",
            *project_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectError::FetchProject(e, project_id))?;

    let project_group_id = if let Some(x) = project {
        x.project_group_id
    } else {
        return Err(ProjectError::Forbidden(project_id, character_id));
    };

    crate::project_group::permission::assert_read_access_check(
            pool,
            project_group_id.into(),
            character_id,
        )
        .await
        .map_err(Into::into)
}
