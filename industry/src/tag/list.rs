use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::tag::Tag;

use crate::AppState;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::tag::error::Result;
use crate::tag::service::list;

/// List Projects
/// 
/// - Alternative route: `/latest/projects`
/// - Alternative route: `/v1/projects`
/// 
/// ---
/// 
/// Lists all projects the user has access to.
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "projects",
    responses(
        (
            body = Vec<Tag>,
            description = "List all projects that match the given filters",
            status = OK,
        ),
        (
            description = "There aren't any projects matching the filter",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity:      ExtractIdentity,
    State(state):  State<AppState>,
) -> Result<impl IntoResponse> {
    let data = list(
            &state.postgres,
            identity.character_id,
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
