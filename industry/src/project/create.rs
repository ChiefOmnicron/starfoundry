use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::project::{CreateProject, CreateProjectResponse};
use starfoundry_lib_industry::ProjectUuid;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::project::error::Result;
use crate::project::service::create;

/// List Groups
/// 
/// - Alternative route: `/latest/projects`
/// - Alternative route: `/v1/projects`
/// 
/// ---
/// 
/// Lists all project groups the user has access to.
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "projects",
    request_body = CreateProjectResponse,
    responses(
        (
            body = ProjectUuid,
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
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Json(project_info): Json<CreateProject>,
) -> Result<impl IntoResponse> {
    let id = create(
            &state.postgres,
            identity.character_id,
            project_info,
        ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateProjectResponse {
                id: id.into(),
            })
        )
    )
}
