use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::{eve_gateway_api_client, AppState};
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::industry_hub::service::IndustryHub;
use crate::project_group::error::Result;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::list_industry_hubs;

/// List Industry Hubs
/// 
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}/industry-hubs`
/// Alternative route: `/v1/project-groups/{ProjectGroupUuid}/industry-hubs`
/// 
/// ---
/// 
/// Fetches all configured industry hubs
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}/industry-hubs",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = Vec<IndustryHub>,
            description = "All industry hubs configured in the project group",
            status = OK,
        ),
        (
            description = "No industry hubs configured",
            status = NO_CONTENT,
        ),
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity:                 ExtractIdentity,
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    let data = list_industry_hubs(
            &state.pool,
            &eve_gateway_api_client()?,
            identity.character_id,
            project_group_uuid,
        )
        .await?;

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
