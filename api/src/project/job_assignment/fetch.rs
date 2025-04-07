use sqlx::PgPool;
use starfoundry_libs_projects::{JobAssignmentGroup, ProjectJobAssignmentUuid, ProjectJobAssignmentService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::api_docs::{BadRequest, InternalServerError};
use crate::project::job_assignment::ProjectJobAssignmentUuidPath;
use crate::ReplyError;

/// /projects/job-assignments/{projectJobAssignmentUuid}
/// 
/// Fetches the given job assignment and returns the jobs grouped by their
/// category_id and group_id.
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
/// 
#[utoipa::path(
    get,
    operation_id = "project_job_assignments_fetch",
    path = "/projects/job-assignments/{projectJobAssignmentUuid}",
    tag = "project-assignments",
    params(
        ProjectJobAssignmentUuidPath,
    ),
    responses(
        (
            body = JobAssignmentGroup,
            description = "List of all jobs that can be started in the assignment, sorted by the project",
            status = OK,
        ),
        BadRequest,
        InternalServerError,
    ),
)]
pub async fn fetch(
    pool:            PgPool,
    assignment_uuid: ProjectJobAssignmentUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectJobAssignmentService::new(assignment_uuid);

    match project.fetch(
        &pool,
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
