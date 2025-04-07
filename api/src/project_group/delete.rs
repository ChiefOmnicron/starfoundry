use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroupUuid, ProjectGroupService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use super::ProjectGroupUuidPath;

/// /project-groups/{projectGroupUuid}/members
/// 
/// Fetches all members of a group
/// 
#[utoipa::path(
    delete,
    operation_id = "project_groups_delete",
    path = "/project-groups/{projectGroupUuid}",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        (
            description = "Members of the group",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn delete(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    let project_group = ProjectGroupService::new(project_group_uuid);

    match project_group.delete(
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
