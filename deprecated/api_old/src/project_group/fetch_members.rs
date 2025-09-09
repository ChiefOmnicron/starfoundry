mod member;
mod service;

use sqlx::PgPool;
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::project_group::ProjectGroupUuid;
use crate::project_group::error::Error;

use super::ProjectGroupUuidPath;

pub use self::member::*;
pub use self::service::*;

/// /project-groups/{projectGroupUuid}/members
/// 
/// Fetches all members of a group
/// 
#[utoipa::path(
    get,
    operation_id = "project_groups_fetch_members",
    path = "/project-groups/{projectGroupUuid}/members",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        (
            body = Vec<ProjectGroupMember>,
            description = "Members of the group",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn fetch_members_api(
    pool:               PgPool,
    _:                  Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    match fetch_members(
        &pool,
        project_group_uuid,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
