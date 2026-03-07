use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::{AppState, eve_gateway_api_client};
use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::{ProjectMarketGroup, list_market};

/// List Market
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/market`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market`
/// 
/// ---
/// 
/// Lists all materials that need to be bought
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/market",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<ProjectMarketGroup>,
            description = "List all materials for a project",
            status = OK,
        ),
        (
            description = "There aren't any materials required for the project",
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
    _identity:        ExtractIdentity,
    State(state):     State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let data = list_market(
            &state.postgres,
            project_id,
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
