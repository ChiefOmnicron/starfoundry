use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroupUuid, ProjectGroupService};
use starfoundry_libs_types::CharacterId;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::auth::CharacterIdPath;
use crate::project_group::ProjectGroupUuidPath;

/// /api/v1/project-groups/{projectGroupUuid}/members
/// 
/// An external user accept the invitation to a project
/// 
#[utoipa::path(
    put,
    operation_id = "project_groups_accept_member",
    path = "/api/v1/project-groups/{projectGroupUuid}/members/{characterIdPath}/accept",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
        CharacterIdPath,
    ),
    responses(
        (
            description = "The member was accepted",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn accept_member(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
    character_id:       CharacterId,
) -> Result<impl Reply, Rejection> {
    let project_group = ProjectGroupService::new(project_group_uuid);

    match project_group.accept_member(
        &pool,
        identity.character_id(),
        character_id,
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
