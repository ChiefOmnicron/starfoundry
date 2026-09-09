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
/// - Alternative route: `/v1/project/{ProjectUuid}/sub-project`
/// - Alternative route: `/latest/project/{ProjectUuid}/sub-project`
/// 
/// ---
/// 
/// Updates the sub-project of the project
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{ProjectUuid}/sub-project",
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
    Json(sub_projects): Json<Vec<ProjectUuid>>,
) -> Result<impl IntoResponse> {
    let mut transaction = state
        .postgres
        .begin()
        .await
        .map_err(ProjectError::TransactionError)?;

    sqlx::query!("
            DELETE FROM project_sub_project
            WHERE project_id = $1
        ",
            *project_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Update)?;

    sqlx::query!("
            INSERT INTO project_sub_project
            (
                project_id,
                sub_project_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[]
            )
        ",
            *project_id,
            &sub_projects.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Update)?;

    transaction
        .commit()
        .await
        .map_err(ProjectError::TransactionError)?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
