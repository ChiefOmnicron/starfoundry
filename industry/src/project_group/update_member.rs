use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::project_group::permission::ProjectGroupPermissionCode;
use crate::{AppState, ProjectGroupUuid};

/// Update Members
///
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}/members`
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}/members`
///
/// ---
///
/// Updates a default market
///
/// ## Security
/// - authenticated
/// - project_group:write
#[utoipa::path(
    put,
    path = "/{ProjectGroupUuid}/members",
    tag = "Project Groups",
    request_body = Vec<UpdateMemberRequest>,
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            description = "The group was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    Json(update_info): Json<Vec<UpdateMemberRequest>>,
) -> Result<impl IntoResponse> {
    update_member(&state.postgres, project_group_uuid, update_info).await?;

    Ok((StatusCode::NO_CONTENT,))
}

pub async fn update_member(
    pool: &PgPool,
    project_group_uuid: ProjectGroupUuid,
    update_info: Vec<UpdateMemberRequest>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionError)?;

    sqlx::query!(
        "
            DELETE FROM project_group_member
            WHERE project_group_id = $1
        ",
        *project_group_uuid,
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| {
        ProjectGroupError::DeleteIndustryHubs(e, project_group_uuid)
    })?;

    sqlx::query!(
        "
            INSERT INTO project_group_member
            (
                project_group_id,
                permission,
                character_id
            )
            SELECT $1, $2, * FROM UNNEST(
                $3::INTEGER[]
            )
            ON CONFLICT (project_group_id, character_id) DO NOTHING
        ",
        *project_group_uuid,
        // TODO: proper permission
        *ProjectGroupPermissionCode::Owner,
        &update_info
            .into_iter()
            .map(|x| *x.character_id)
            .collect::<Vec<_>>(),
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| ProjectGroupError::UpdateMembers(e, project_group_uuid))?;

    transaction
        .commit()
        .await
        .map_err(ProjectGroupError::TransactionError)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMemberRequest {
    character_id: CharacterId,
}
