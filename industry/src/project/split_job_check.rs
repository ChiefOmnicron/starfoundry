use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::{AppState, eve_gateway_api_client};
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::{SplitJobRequest, split_job_check};

/// List Groups
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/split-job/check`
/// - Alternative route: `/v1/projects/{ProjectUuid}/split-job/check`
/// 
/// ---
/// 
/// Checks how splitting a job will affect the project
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    put,
    path = "/{ProjectUuid}/split-job/check",
    tag = "projects",
    request_body = SplitJobRequest,
    responses(
        (
            body = ProjectUuid,
            description = "Changes that will are required",
            status = OK,
        ),
        (
            description = "No changes required",
            status = NO_CONTENT,
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
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Path(project_id):   Path<ProjectUuid>,
    Json(split):        Json<SplitJobRequest>,
) -> Result<impl IntoResponse> {
    let change = split_job_check(
            &state.postgres,
            identity.character_id,
            &eve_gateway_api_client()?,
            project_id,
            split,
        ).await?;

    let status;
    if change.jobs.is_empty() && change.materials.is_empty() {
        status = StatusCode::NO_CONTENT;
    } else {
        status = StatusCode::OK;
    }

    Ok(
        (
            status,
            Json(change)
        )
    )
}
