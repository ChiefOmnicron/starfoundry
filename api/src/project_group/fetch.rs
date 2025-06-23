use serde::Serialize;
use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroupService, ProjectGroupUuid};
use utoipa::ToSchema;
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project_group::ProjectGroupUuidPath;
use uuid::Uuid;

/// /project-groups/{projectGroupUuid}
/// 
/// Fetches additional information about a project group
/// 
#[utoipa::path(
    get,
    operation_id = "project_groups_fetch",
    path = "/project-groups/{projectGroupUuid}",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        (
            body = ProjectGroupResponse,
            description = "Information about the group",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
)]
pub async fn fetch(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    let project_group = ProjectGroupService::new(project_group_uuid);

    match project_group.fetch(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_projects::Error::ProjectGroupNotFound(_)) => {
            Err(ReplyError::NotFound.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "022e57de-0571-43d1-b9c6-4a0d97940177",
        "name": "My cool project group",
        "members": 1,
        "projects": 10,
        "description": "Contains some really cool projects"
    })
)]
pub struct ProjectGroupResponse {
    /// UUID of the group
    pub id:          Uuid,
    /// Name of the group
    pub name:        String,
    /// Number of members in the group
    pub members:     i64,
    /// Number of projects in the group
    pub projects:    i64,

    /// Description of the group
    pub description: Option<String>,
}
