use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::{AppState, eve_gateway_api_client};
use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::job_assignments::service::{ProjectJobAssignmentGroup, list_job_assignments};
use crate::job_assignments::error::Result;
use crate::job_assignments::JobAssignmentUuid;

/// List Job Assignments
/// 
/// - Alternative route: `/latest/job-assignments/{JobAssignmentUuid}`
/// - Alternative route: `/v1/job-assignments/{JobAssignmentUuid}`
/// 
/// ---
/// 
#[utoipa::path(
    get,
    path = "/{JobAssignmentUuid}",
    tag = "job_assignment",
    params(
        JobAssignmentUuid,
    ),
    responses(
        (
            body = Vec<ProjectJobAssignmentGroup>,
            description = "List all jobs for the given project",
            status = OK,
        ),
        NotFound,
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):           State<AppState>,
    Path(assignment_id):    Path<JobAssignmentUuid>,
) -> Result<impl IntoResponse> {
    let data = list_job_assignments(
            &state.postgres,
            assignment_id,
            &eve_gateway_api_client()?,
        ).await?;

    if data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(data),
            )
            .into_response()
        )
    }
}
