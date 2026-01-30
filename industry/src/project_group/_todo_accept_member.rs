use sqlx::PgPool;
use starfoundry_lib_projects::ProjectGroupService;
use starfoundry_lib_types::CharacterId;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::auth::CharacterIdPath;
use crate::project_group::{ProjectGroupUuid, ProjectGroupUuidPath};

/// /project-groups/{projectGroupUuid}/members
/// 
/// An external user accept the invitation to a project
/// 
#[utoipa::path(
    put,
    operation_id = "project_groups_accept_member",
    path = "/project-groups/{projectGroupUuid}/members/{characterIdPath}/accept",
    tag = "Project Groups",
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
        Err(starfoundry_lib_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
