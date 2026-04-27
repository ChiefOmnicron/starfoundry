use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::project::error::Result;
use crate::project::ProjectAssignmentUuid;
use crate::project::service::{ProjectJobUuid, update_job_assignment};

/// Update Project Job
/// 
/// - Alternative route: `/v1/projects/job-assignments/{ProjectAssignmentUuid}/{ProjectJobUuid}`
/// - Alternative route: `/latest/projects/job-assignments/{ProjectAssignmentUuid}/{ProjectJobUuid}`
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
    path = "/job-assignments/{ProjectAssignmentUuid}/{ProjectJobUuid}",
    tag = "Project",
    params(
        ProjectAssignmentUuid,
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
    Path((assignment_id, job_id)):  Path<(ProjectAssignmentUuid, ProjectJobUuid)>,
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
