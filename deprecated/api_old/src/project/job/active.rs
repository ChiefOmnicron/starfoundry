use sqlx::PgPool;
use starfoundry_lib_projects::{ActiveJob, ProjectUuid, ProjectService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{Identity, ReplyError};
use crate::project::ProjectUuidPath;
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};

/// /projects/{projectUuid}/jobs/active
/// 
/// List of all jobs that are currently running of the project.
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_active_jobs",
    path = "/projects/{projectUuid}/jobs/active",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    responses(
        (
            body = Vec<ActiveJob>,
            description = "List of all jobs that are currently active for the project",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn active(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.active_jobs(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => {
            Ok(
                warp::reply::with_status(
                    warp::reply::json(&x),
                    StatusCode::OK,
                )
            )
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
