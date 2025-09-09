use sqlx::PgPool;
use starfoundry_lib_projects::ProjectGroupService;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::project_group::{ProjectGroupUuid, ProjectGroupUuidPath};

/// /project-groups/{projectGroupUuid}/members/invite
/// 
/// An external user accept the invitation to a project
/// 
#[utoipa::path(
    put,
    operation_id = "project_groups_accept_invite",
    path = "/project-groups/{projectGroupUuid}/members/invite",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        (
            description = "The invite was accepted",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn accept_invite(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    let project_group = ProjectGroupService::new(project_group_uuid);

    match project_group.accept_invite(
        &pool,
        identity.character_id(),
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
