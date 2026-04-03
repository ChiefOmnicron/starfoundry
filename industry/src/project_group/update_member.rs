use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::project_group::error::Result;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::{UpdateMemberRequest, update_member};

/// Update Members
/// 
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}/members`
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}/members`
/// 
/// ---
/// 
/// Updates a default market
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{ProjectGroupUuid}/members",
    tag = "Project Groups",
    request_body = Vec<UpdateMemberRequest>,
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            description = "The group was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    Json(update_info):        Json<Vec<UpdateMemberRequest>>,
) -> Result<impl IntoResponse> {
    update_member(
        &state.postgres,
        project_group_uuid,
        update_info,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
