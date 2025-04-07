use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroupUuid, ProjectGroupService, UpdateProjectGroupMember};
use starfoundry_libs_types::CharacterId;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::auth::CharacterIdPath;
use crate::project_group::ProjectGroupUuidPath;

/// /project-groups/{projectGroupUuid}/members/{characterId}
/// 
/// Updates the member permission of a group member
/// 
#[utoipa::path(
    put,
    operation_id = "project_groups_update_members",
    path = "/project-groups/{projectGroupUuid}/members/{characterId}",
    tag = "project-groups",
    request_body = UpdateProjectGroupMember,
    params(
        ProjectGroupUuidPath,
        CharacterIdPath
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
pub async fn update_member(
    pool:                PgPool,
    identity:            Identity,
    project_group_uuid:  ProjectGroupUuid,
    member_character_id: CharacterId,
    info:                UpdateProjectGroupMember,
) -> Result<impl Reply, Rejection> {
    let project_group = ProjectGroupService::new(project_group_uuid);

    match project_group.update_member(
        &pool,
        identity.character_id(),
        member_character_id,
        info,
    ).await {
        Ok(x) => {
            let response = warp::reply::with_status(
                warp::reply::json(&x),
                StatusCode::NO_CONTENT,
            );
            Ok(response)
        },
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_projects::Error::ProjectGroupNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
