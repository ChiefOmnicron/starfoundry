use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::ProjectUuid;

use crate::AppState;
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project::error::Result;
use crate::project::service::{ProjectCost, fetch_cost};

/// Fetch Cost
/// 
/// - Alternative route: `/latest/structures/{ProjectUuid}/cost`
/// - Alternative route: `/v1/structures/{ProjectUuid}/cost`
/// 
/// ---
/// 
/// Gets the cost of the project
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/cost",
    tag = "Projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = ProjectCost,
            description = "Project costs",
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
    let entry = fetch_cost(
            &state.postgres,
            identity.character_id,
            project_id,
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
