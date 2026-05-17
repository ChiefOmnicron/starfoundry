use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::{ProjectJobUuid, ProjectUuid};

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::project::error::Result;
use crate::project::service::delete_job;

/// Delete project
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/jobs/{ProjectJobUuid}`
/// - Alternative route: `/v1/projects/{ProjectUuid}/jobs/{ProjectJobUuid}`
/// 
/// ---
/// 
/// Fetches information about a structure
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    delete,
    path = "/{ProjectUuid}/jobs/{ProjectJobUuid}",
    tag = "Project",
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "Project job was deleted",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):                   State<AppState>,
    Path((project_id, job_id)):     Path<(ProjectUuid, ProjectJobUuid)>,
) -> Result<impl IntoResponse> {
    delete_job(
            &state.postgres,
            project_id,
            job_id,
        )
        .await?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(())
        )
        .into_response()
    )
}
