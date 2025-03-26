use sqlx::PgPool;
use starfoundry_libs_projects::{CreateProjectGroup, ProjectGroupUuid, ProjectGroupService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};

/// /api/v1/project-groups/{projectGroupUuid}/members/invite
/// 
/// An external user accept the invitation to a project
/// 
#[utoipa::path(
    post,
    operation_id = "project_groups_create",
    path = "/api/v1/project-groups",
    tag = "project-groups",
    request_body = CreateProjectGroup,
    responses(
        (
            body = ProjectGroupUuid,
            description = "The group was created",
            status = CREATED,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn create(
    pool:     PgPool,
    identity: Identity,
    info:     CreateProjectGroup,
) -> Result<impl Reply, Rejection> {
    match ProjectGroupService::create(
        &pool,
        identity.character_id(),
        info,
    ).await {
        Ok(x) => {
            let response = warp::reply::with_status(
                warp::reply::json(&x),
                StatusCode::CREATED,
            );
            Ok(response)
        },
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_projects::Error::ProjectGroupNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
