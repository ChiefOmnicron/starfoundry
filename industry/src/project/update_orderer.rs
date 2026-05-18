use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::ProjectUuid;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::project::error::{ProjectError, Result};

/// Update
/// 
/// - Alternative route: `/v1/project/{ProjectUuid}/orderer`
/// - Alternative route: `/latest/project/{ProjectUuid}/orderer`
/// 
/// ---
/// 
/// Updates the orderer of the project
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{ProjectUuid}/orderer",
    tag = "Project",
    request_body = String,
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
    Json(orderer):      Json<String>,
) -> Result<impl IntoResponse> {
    sqlx::query!("
            UPDATE project
            SET orderer = $2
            WHERE id = $1
        ",
            *project_id,
            orderer,
        )
        .execute(&state.postgres)
        .await
        .map_err(ProjectError::Update)?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
