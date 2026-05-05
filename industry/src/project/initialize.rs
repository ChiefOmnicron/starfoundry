use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use starfoundry_lib_industry::{ProjectUuid, SolutionUuid};
use utoipa::ToSchema;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::project::error::Result;
use crate::project::service::initialize;

/// Update Default Blacklist
/// 
/// - Alternative route: `/v1/projects/{ProjectUuid}/initialize`
/// - Alternative route: `/latest/projects/{ProjectUuid}/initialize`
/// 
/// ---
/// 
/// Updates the project jobs
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{ProjectUuid}/initialize",
    tag = "Project",
    request_body = InitializeProject,
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "The group was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):       State<AppState>,
    Path(project_id):   Path<ProjectUuid>,
    Json(info):         Json<InitializeProject>,
) -> Result<impl IntoResponse> {
    initialize(
        &state.postgres,
        project_id,
        info.solution_id,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct InitializeProject {
    pub solution_id: SolutionUuid,
}
