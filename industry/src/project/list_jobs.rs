use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::ProjectUuid;

use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::{AppState, eve_gateway_api_client};
use crate::project::error::Result;
use crate::project::service::{ProjectJob, ProjectJobFilter, list_jobs};

/// List Jobs
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/jobs`
/// - Alternative route: `/v1/projects/{ProjectUuid}/jobs`
/// 
/// ---
/// 
/// Lists all jobs that belong to the project
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/jobs",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<ProjectJob>,
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
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Path(project_id):   Path<ProjectUuid>,
    Query(filter):      Query<ProjectJobFilter>,
) -> Result<impl IntoResponse> {
    let data = list_jobs(
            &state.postgres,
            identity.character_id,
            project_id,
            &eve_gateway_api_client()?,
            filter,
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
