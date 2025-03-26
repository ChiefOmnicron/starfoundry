use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroupUuid, ProjectGroupDefault, ProjectGroupService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::project_group::ProjectGroupUuidPath;

/// /api/v1/project-groups/{projectGroupUuid}/default
/// 
/// Updates the defaults for a project group
/// 
#[utoipa::path(
    put,
    operation_id = "project_groups_update_default",
    path = "/api/v1/project-groups/{projectGroupUuid}/default",
    tag = "project-groups",
    request_body = ProjectGroupDefault,
    params(
        ProjectGroupUuidPath,
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
pub async fn update_default(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
    info:               ProjectGroupDefault,
) -> Result<impl Reply, Rejection> {
    let project_group = ProjectGroupService::new(project_group_uuid);

    match project_group.update_default(
        &pool,
        identity.character_id(),
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
