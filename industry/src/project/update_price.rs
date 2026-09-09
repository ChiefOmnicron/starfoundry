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
/// - Alternative route: `/v1/project/{ProjectUuid}/price`
/// - Alternative route: `/latest/project/{ProjectUuid}/price`
/// 
/// ---
/// 
/// Updates the price of the project
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{ProjectUuid}/price",
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
    Json(sell_price):   Json<f64>,
) -> Result<impl IntoResponse> {
    sqlx::query!("
            UPDATE project
            SET sell_price = $2
            WHERE id = $1
        ",
            *project_id,
            sell_price,
        )
        .execute(&state.postgres)
        .await
        .map_err(ProjectError::Update)?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
