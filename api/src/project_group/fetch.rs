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
/// Alternative route: `/v1/project-groups/{projectGroupUuid}`
/// 
/// ---
/// 
/// Fetches information about a project group
/// 
/// ## Security
/// - authenticated
/// - project_group: read
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

    match dbg!(project_group.fetch(
        &pool,
        identity.character_id(),
    ).await) {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(starfoundry_libs_projects::Error::ProjectGroupNotFound(_)) => {
            Err(ReplyError::NotFound.into())
        },
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
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

#[cfg(test)]
mod fetch_project_group_test {
    use sqlx::PgPool;
    use starfoundry_libs_projects::ProjectGroupUuid;
    use warp::Filter;
    use warp::http::StatusCode;

    use crate::test_util::credential_cache;
    use crate::{with_identity, with_pool};

    #[sqlx::test(
        fixtures("fetch"),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let filter = warp::any()
            .clone()
            .and(with_pool(pool.clone()))
            .and(with_identity(pool.clone(), credential_cache(pool.clone()).await))
            .and(warp::path!("project-groups" / ProjectGroupUuid))
            .and(warp::get())
            .and_then(super::fetch)
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(
        fixtures("fetch"),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn no_entry_with_default_uuid(
        pool: PgPool,
    ) {
        let filter = warp::any()
            .clone()
            .and(with_pool(pool.clone()))
            .and(with_identity(pool.clone(), credential_cache(pool.clone()).await))
            .and(warp::path!("project-groups" / ProjectGroupUuid))
            .and(warp::get())
            .and_then(super::fetch)
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups/00000000-0000-0000-0000-000000000000")
            .method("GET")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
