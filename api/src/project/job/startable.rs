use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectService, ProjectUuid, StartableJobGroup};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{Identity, ReplyError};
use crate::project::ProjectUuidPath;
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};

/// /api/v1/projects/{projectUuid}/jobs/startable
/// 
/// Lists all jobs that can be started, either because they require base
/// materials, or because their dependencies are done.
/// The jobs are grouped by their category_id and group_id
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
    operation_id = "project_startable_jobs",
    path = "/api/v1/projects/{projectUuid}/jobs/startable",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    responses(
        (
            body = Vec<StartableJobGroup>,
            description = "List of all jobs that can be started",
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
pub async fn startable(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.startable_jobs(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => {
            Ok(
                warp::reply::with_status(
                    warp::reply::json(&x.into_group()),
                    StatusCode::OK,
                )
            )
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
