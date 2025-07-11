use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectJobAssignmentUuid, ProjectJobAssignmentService, ProjectJobUuid};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::api_docs::{BadRequest, InternalServerError, NoContent};
use crate::project::job::ProjectJobUuidPath;
use crate::project::job_assignment::ProjectJobAssignmentUuidPath;
use crate::ReplyError;

/// /projects/job-assignments/{projectJobAssignmentUuid}/{projectJobUuid}/state
/// 
/// Sets the job status to`started`.
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    put,
    operation_id = "project_job_assignments_update_job_state",
    path = "/projects/job-assignments/{projectJobAssignmentUuid}/{projectJobUuid}/state",
    tag = "project-assignments",
    params(
        ProjectJobAssignmentUuidPath,
        ProjectJobUuidPath,
    ),
    responses(
        NoContent,
        BadRequest,
        InternalServerError,
    ),
)]
pub async fn update_job_state(
    pool:            PgPool,
    assignment_uuid: ProjectJobAssignmentUuid,
    job_uuid:        ProjectJobUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectJobAssignmentService::new(assignment_uuid);

    match project.update_job_state(
        &pool,
        job_uuid,
    ).await {
        Ok(_) => {
            Ok(
                warp::reply::with_status(
                    warp::reply::json(&()),
                    StatusCode::NO_CONTENT,
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
