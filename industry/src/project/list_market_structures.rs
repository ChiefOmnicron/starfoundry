use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::Structure;

use crate::{AppState, eve_gateway_api_client};
use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::list_market_structures;

/// List Market
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/market/structures`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market/structures`
/// 
/// ---
/// 
/// Lists all default markets for a project
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/market/structures",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<Structure>,
            description = "List all market structures for a project",
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
    identity:         ExtractIdentity,
    State(state):     State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let data = list_market_structures(
            &state.postgres,
            identity.character_id,
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
