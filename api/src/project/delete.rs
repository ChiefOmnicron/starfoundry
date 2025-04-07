use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectService, ProjectUuid};
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{Forbidden, InternalServerError, NoContent, Unauthorized};
use crate::project::ProjectUuidPath;

/// /projects/{projectUuid}
/// 
/// Delete a project
/// 
/// If you don't have access to a project or the project cannot be found with the
/// given UUID, it will return 401.
/// 
/// ## Security
/// - authenticated
/// - project:owner
/// 
#[utoipa::path(
    delete,
    operation_id = "project_delete",
    path = "/projects/{projectUuid}",
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
pub async fn delete(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
) -> Result<impl Reply, Rejection> {
    let service = ProjectService::new(project_uuid);

    match service.delete(
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
        Err(starfoundry_libs_projects::Error::ProjectNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
