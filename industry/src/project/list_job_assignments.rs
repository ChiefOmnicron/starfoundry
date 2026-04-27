use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::{AppState, eve_gateway_api_client};
use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::project::{ProjectAssignmentUuid, ProjectUuid};
use crate::project::error::Result;
use crate::project::service::{ProjectJobAssignmentGroup, list_job_assignments};

/// List Job Assignments
/// 
/// - Alternative route: `/latest/projects/job-assignments/{ProjectAssignmentUuid}`
/// - Alternative route: `/v1/projects/job-assignments/{ProjectAssignmentUuid}`
/// 
/// ---
/// 
#[utoipa::path(
    get,
    path = "/job-assignments/{ProjectAssignmentUuid}",
    tag = "projects",
    params(
        ProjectUuid,
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
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):           State<AppState>,
    Path(assignment_id):    Path<ProjectAssignmentUuid>,
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
