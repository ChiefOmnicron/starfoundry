use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::AppState;
use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::project_group::permission::ProjectGroupPermissionCode;

pub async fn assert_owner(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    identity:                 ExtractIdentity,
    request:                  Request,
    next:                     Next,
) -> Result<impl IntoResponse> {
    assert_owner_check(
            &state.pool,
            project_group_uuid,
            identity.character_id,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_owner_check(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
    character_id:       CharacterId,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT pg.id
            FROM project_group pg
            JOIN project_group_member pgm ON pgm.project_group_id = pg.id
            WHERE pg.id = $1
                AND pgm.character_id = $2
                AND permission & $3 = $3
        ",
            *project_group_uuid,
            *character_id,
            *ProjectGroupPermissionCode::Owner,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchGroupPermissions(e, project_group_uuid))?;

    if result.is_none() {
        return Err(ProjectGroupError::Forbidden(project_group_uuid, character_id));
    } else {
        Ok(())
    }
}
