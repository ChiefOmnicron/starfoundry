use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::project::Project;
use starfoundry_lib_industry::ProjectUuid;

use crate::{AppState, eve_gateway_api_client};
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project::error::Result;
use crate::project::service::fetch;

/// Fetch Structure
/// 
/// - Alternative route: `/latest/structures/{ProjectUuid}`
/// - Alternative route: `/v1/structures/{ProjectUuid}`
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
    get,
    path = "/{ProjectUuid}",
    tag = "Structures",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Project,
            description = "Information about the structure",
            status = OK,
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
    identity:         ExtractIdentity,
    State(state):     State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let entry = fetch(
            &state.postgres,
            identity.character_id,
            project_id,
            &eve_gateway_api_client()?,
        )
        .await?;

    if let Some(x) = entry {
        Ok(
            (
                StatusCode::OK,
                Json(x)
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(())
            )
            .into_response()
        )
    }
}
