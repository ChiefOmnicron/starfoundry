use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use starfoundry_lib_industry::ProjectJobUuid;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::job_assignments::JobAssignmentUuid;
use crate::job_assignments::error::Result;
use crate::job_assignments::service::update_job_assignment;

/// Update Project Job
/// 
/// - Alternative route: `/v1/job-assignments/{JobAssignmentUuid}/{ProjectJobUuid}`
/// - Alternative route: `/latest/job-assignments/{JobAssignmentUuid}/{ProjectJobUuid}`
/// 
/// ---
/// 
/// Updates the project jobs
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{JobAssignmentUuid}/{ProjectJobUuid}",
    tag = "job_assignments",
    params(
        JobAssignmentUuid,
        ProjectJobUuid,
    ),
    responses(
        (
            description = "The job was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):                   State<AppState>,
    Path((assignment_id, job_id)):  Path<(JobAssignmentUuid, ProjectJobUuid)>,
) -> Result<impl IntoResponse> {
    update_job_assignment(
        &state.postgres,
        assignment_id,
        job_id,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
