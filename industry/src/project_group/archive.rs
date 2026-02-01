use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::project_group::error::Result;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::archive;

/// Delete Group
/// 
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}/archive`
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}/archive`
/// 
/// ---
/// 
/// Deletes the group
/// 
/// ## Security
/// - authenticated
/// - project_group:owner
/// 
#[utoipa::path(
    put,
    path = "/{ProjectGroupUuid}/archive",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            description = "The group was successfully archived",
            status = NO_CONTENT,
        ),
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    archive(
        &state.pool,
        project_group_uuid,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
        ()
    ))
}
