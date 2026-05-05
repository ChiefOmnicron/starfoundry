use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::project::UpdateProject;
use starfoundry_lib_industry::ProjectUuid;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::project::error::Result;
use crate::project::service::update;

/// Update
/// 
/// - Alternative route: `/v1/project/{ProjectUuid}`
/// - Alternative route: `/latest/project/{ProjectUuid}`
/// 
/// ---
/// 
/// Updates the miscellaneous entries
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{ProjectUuid}",
    tag = "Project",
    request_body = UpdateProject,
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "The project was updated",
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
    Json(update_info):  Json<UpdateProject>,
) -> Result<impl IntoResponse> {
    update(
        &state.postgres,
        project_id,
        update_info,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
