use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::{AppState, eve_gateway_api_client};
use crate::project::error::Result;
use crate::project::service::{ProjectJobAllGroup, list_all_jobs};
use crate::project::ProjectUuid;

/// List All Jobs
/// 
/// - Alternative route: `/latest/projects/jobs`
/// - Alternative route: `/v1/projects/jobs`
/// 
/// ---
/// 
/// Lists all startable jobs over all projects
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    path = "/jobs",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<ProjectJobAllGroup>,
            description = "List all jobs for the given project",
            status = OK,
        ),
        (
            description = "There aren't any jobs associated with the project",
            status = NO_CONTENT,
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
    identity:       ExtractIdentity,
    State(state):   State<AppState>,
) -> Result<impl IntoResponse> {
    let data = list_all_jobs(
            &state.postgres,
            identity.character_id,
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
