use sqlx::PgPool;
use starfoundry_libs_projects::{Project, ProjectService, ProjectUuid};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::project::ProjectUuidPath;

/// /projects/{projectUuid}
/// 
/// Fetches general information about a project by it's UUID.
/// 
/// If you don't have access to a project or the project cannot be found with the
/// given UUID, it will return 401.
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    operation_id = "projects_fetch",
    path = "/projects/{projectUuid}",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    responses(
        (
            body = Project,
            content_type = "application/json",
            description = "General information about the project",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security (
        ("jwt" = ["project:read"])
    ),
)]
pub async fn fetch(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.fetch(
        &pool,
        identity.character_id(),
    ).await {
        Ok(Some(x)) => Ok(warp::reply::json(&x)),
        Ok(None) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_projects::Error::ProjectNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
