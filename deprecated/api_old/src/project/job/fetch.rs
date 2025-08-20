use sqlx::PgPool;
use starfoundry_lib_projects::{FetchJobFilter, JobGroup, ProjectService, ProjectUuid};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::error::ReplyError;
use crate::Identity;
use crate::project::ProjectUuidPath;

/// /projects/{projectUuid}/jobs
/// 
/// Lists all industry jobs that are required to finish the project. The jobs
/// are grouped by their category_id and group_id
/// 
/// ## Order:
/// - Intermediate Reactions
/// - Composite Reactions
/// - Biochem Reactions
/// - Hybrid Reactions
/// - Construction Components
/// - Advanced Capital Construction Components
/// - Capital Construction Components
/// - Tools
/// - T1 Stuff
/// - T2 Stuff
/// - Charges
/// - Ships
/// - Unknown
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_job_fetch",
    path = "/projects/{projectUuid}/jobs",
    tag = "projects",
    params(
        ProjectUuidPath,
        FetchJobFilter,
    ),
    responses(
        (
            body = JobGroup,
            content_type = "application/json",
            description = "List of jobs grouped by their category_id and group_id",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn fetch(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    filter:       FetchJobFilter,
) -> Result<impl Reply, Rejection> {
    let service = ProjectService::new(project_uuid);

    match service.fetch_jobs(
        &pool,
        identity.character_id(),
        filter.clone(),
    ).await {
        Ok(x) => {
            if filter.grouped {
                Ok(warp::reply::json(&x.into_group()))
            } else {
                Ok(warp::reply::json(&x))
            }
        },
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
