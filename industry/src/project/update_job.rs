use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::{ProjectJobUuid, ProjectUuid};

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::project::service::{UpdateProjectJob, update_job};
use crate::project::error::Result;

/// Update Project Job
/// 
/// - Alternative route: `/v1/projects/{ProjectUuid}/jobs/{ProjectJobUuid}`
/// - Alternative route: `/latest/projects/{ProjectUuid}/jobs/{ProjectJobUuid}`
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
    path = "/{ProjectUuid}/jobs/{ProjectJobUuid}",
    tag = "Project",
    request_body = UpdateProjectJob,
    params(
        ProjectUuid,
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
    State(state):                       State<AppState>,
    Path((project_id, project_job_id)): Path<(ProjectUuid, ProjectJobUuid)>,
    Json(update_info):                  Json<UpdateProjectJob>,
) -> Result<impl IntoResponse> {
    update_job(
        &state.postgres,
        project_id,
        project_job_id,
        update_info,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
