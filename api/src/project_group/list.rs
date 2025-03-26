use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroupUuid, ProjectGroupFilter, ProjectGroupService};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};

/// /api/v1/project-groups
/// 
/// Lists all project groups the user has access to
/// 
#[utoipa::path(
    get,
    operation_id = "project_groups_fetch",
    path = "/api/v1/project-groups",
    tag = "project-groups",
    responses(
        (
            body = Vec<ProjectGroupUuid>,
            description = "Information about the group",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn list(
    pool:     PgPool,
    identity: Identity,
    filter:   ProjectGroupFilter,
) -> Result<impl Reply, Rejection> {
    match ProjectGroupService::list(
        &pool,
        identity.character_id(),
        filter,
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
