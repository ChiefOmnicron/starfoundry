use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::{UpdateMiscRequest, update_misc};

/// Update Misc
/// 
/// - Alternative route: `/v1/project/{ProjectUuid}/misc`
/// - Alternative route: `/latest/project/{ProjectUuid}/misc`
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
    path = "/{ProjectUuid}/misc",
    tag = "Project",
    request_body = UpdateMiscRequest,
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
    Json(update_info):  Json<Vec<UpdateMiscRequest>>,
) -> Result<impl IntoResponse> {
    update_misc(
        &state.postgres,
        project_id,
        update_info,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
