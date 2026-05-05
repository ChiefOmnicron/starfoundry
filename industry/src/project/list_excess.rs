use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::project::ProjectExcess;
use starfoundry_lib_industry::ProjectUuid;

use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::{AppState, eve_gateway_api_client};
use crate::project::error::Result;
use crate::project::service::list_excess;

/// List Excess
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/excess`
/// - Alternative route: `/v1/projects/{ProjectUuid}/excess`
/// 
/// ---
/// 
/// Lists all excess entries that belong to the project
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/excess",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<ProjectExcess>,
            description = "List all misc entries for a project",
            status = OK,
        ),
        (
            description = "There aren't any misc entries",
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
    State(state):     State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let data = list_excess(
            &state.postgres,
            &eve_gateway_api_client()?,
            project_id,
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
