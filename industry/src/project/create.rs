use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::ToSchema;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::{CreateProject, create};

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

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateProjectResponse {
    id: ProjectUuid,
}
