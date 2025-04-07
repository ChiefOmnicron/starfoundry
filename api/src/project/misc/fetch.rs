use sqlx::PgPool;
use starfoundry_libs_projects::{Misc, ProjectService, ProjectUuid};
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::{BadRequest, Forbidden, InternalServerError, Unauthorized};
use crate::error::ReplyError;
use crate::Identity;
use crate::project::ProjectUuidPath;

/// /projects/{projectUuid}/misc
/// 
/// Lists all misc entries
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_misc_fetch",
    path = "/projects/{projectUuid}/misc",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    responses(
        (
            body = Vec<Misc>,
            content_type = "application/json",
            description = "List of misc entries",
            status = OK,
        ),
        BadRequest,
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
    let service = ProjectService::new(project_uuid);

    match service.fetch_misc(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => {
            Ok(warp::reply::with_status(
                warp::reply::json(&x),
                StatusCode::OK,
            ))
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
