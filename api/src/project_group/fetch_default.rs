use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroupDefault, ProjectGroupService, ProjectGroupUuid};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use super::ProjectGroupUuidPath;

/// /project-groups/{projectGroupUuid}/default
/// 
/// Fetches the defaults configured by the project group
/// 
#[utoipa::path(
    get,
    operation_id = "project_groups_fetch_default",
    path = "/project-groups/{projectGroupUuid}/default",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        (
            body = ProjectGroupDefault,
            description = "Defaults of the group",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn fetch_default(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    let project_group = ProjectGroupService::new(project_group_uuid);

    match project_group.fetch_default(
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
