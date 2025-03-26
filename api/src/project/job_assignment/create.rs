use sqlx::PgPool;
use starfoundry_libs_projects::{CreateJobAssignment, ProjectJobAssignmentUuid, ProjectJobAssignmentService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{Identity, ReplyError};
use crate::api_docs::{BadRequest, InternalServerError};

/// /api/v1/projects/job-assignments
/// 
/// Creates a new job assignment
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    post,
    operation_id = "project_job_assignments_create",
    path = "/api/v1/projects/job-assignments",
    tag = "project-assignments",
    request_body(
        content = CreateJobAssignment,
        description = "List of jobs that should be in the assignment",
        content_type = "application/json"
    ),
    responses(
        (
            body = ProjectJobAssignmentUuid,
            content_type = "application/json",
            description = "Uuid of the new assignmnent",
            status = CREATED,
        ),
        BadRequest,
        InternalServerError,
    ),
    security (
        ("jwt" = ["project:read"])
    ),
)]
pub async fn create(
    pool:           PgPool,
    identity:       Identity,
    job_assignment: CreateJobAssignment,
) -> Result<impl Reply, Rejection> {
    match ProjectJobAssignmentService::create(
        &pool,
        identity.character_id(),
        job_assignment,
    ).await {
        Ok(x) => {
            Ok(
                warp::reply::with_status(
                    warp::reply::json(&x),
                    StatusCode::CREATED,
                )
            )
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
