use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectUuid, ProjectService};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::project::ProjectUuidPath;
use crate::api_docs::{Forbidden, InternalServerError, NoContent, Unauthorized};

/// /projects/{projectUuid}/permissions/is-owner
/// 
/// Checks if the character is the owner of the project.
/// Any positive HTTP-Statuscode 2xx should be considered as allowed
/// Any non positive HTTP-Statuscode 4xx, 5xx should be considered as forbidden
/// 
/// ## Security
/// - authenticated
/// - project:owner
/// 
#[utoipa::path(
    get,
    operation_id = "project_permission_is_owner",
    path = "/projects/{projectUuid}/permissions/is-owner",
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
    security (
        ("jwt" = ["project:owner"])
    ),
)]
pub async fn is_owner(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.assert_owner(
        &pool,
        identity.character_id(),
    ).await {
        Ok(_) => Ok(warp::reply::json(&())),
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_projects::Error::ProjectNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
