use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroupUuid, ProjectGroup, ProjectGroupService};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::project_group::ProjectGroupUuidPath;

/// /api/v1/project-groups/{projectGroupUuid}
/// 
/// Fetches additional information about a project group
/// 
#[utoipa::path(
    get,
    operation_id = "project_groups_fetch",
    path = "/api/v1/project-groups/{projectGroupUuid}",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        (
            body = ProjectGroup,
            description = "Information about the group",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn fetch(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    let project_group = ProjectGroupService::new(project_group_uuid);

    match project_group.fetch(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
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
