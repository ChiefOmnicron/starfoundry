use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::project::error::Result;
use crate::project::ProjectAssignmentUuid;
use crate::project::service::{CreateProjectJobAssignment, create_job_assignment};

/// List Groups
/// 
/// - Alternative route: `/latest/projects/job-assignments`
/// - Alternative route: `/v1/projects/job-assignments`
/// 
/// ---
/// 
/// Creates a new job assignment
/// 
#[utoipa::path(
    post,
    path = "/job-assignments",
    tag = "projects",
    request_body = Vec<CreateProjectJobAssignment>,
    responses(
        (
            body = CreateProjectJobAssignmentResponse,
            description = "List all projects that match the given filters",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    Json(jobs):     Json<Vec<CreateProjectJobAssignment>>,
) -> Result<impl IntoResponse> {
    let id = create_job_assignment(
            &state.postgres,
            jobs,
        ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateProjectJobAssignmentResponse {
                id: id.into(),
            })
        )
    )
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateProjectJobAssignmentResponse {
    id: ProjectAssignmentUuid,
}
