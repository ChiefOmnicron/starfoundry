use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::delete;

/// Delete project
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}`
/// - Alternative route: `/v1/projects/{ProjectUuid}`
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
    path = "/{ProjectUuid}",
    tag = "Project",
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "Project was deleted",
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
    State(state):     State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    delete(
            &state.postgres,
            project_id,
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
