use sqlx::PgPool;
use starfoundry_lib_projects::{ProjectUuid, ProjectService};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::project::ProjectUuidPath;
use crate::api_docs::{Forbidden, InternalServerError, NoContent, Unauthorized};

/// /projects/{projectUuid}/permissions/can-write
/// 
/// Checks if the character is allowed to write the project.
/// Any positive HTTP-Statuscode 2xx should be considered as allowed
/// Any non positive HTTP-Statuscode 4xx, 5xx should be considered as forbidden
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    get,
    operation_id = "project_permission_can_write",
    path = "/projects/{projectUuid}/permissions/can-write",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    responses(
        NoContent,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn can_write(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.assert_write_access(
        &pool,
        identity.character_id(),
    ).await {
        Ok(_) => Ok(warp::reply::json(&())),
        Err(starfoundry_lib_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_lib_projects::Error::ProjectNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
