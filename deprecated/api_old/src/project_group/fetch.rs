mod project_group;
mod service;

use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project_group::{ProjectGroupUuid, ProjectGroupUuidPath};
use crate::project_group::error::Error;

pub use self::project_group::*;
pub use self::service::*;

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
/// - project_group:read
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
pub async fn fetch_api(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    match   fetch(
        &pool,
        identity.character_id(),
        project_group_uuid,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(Error::NotFound(_)) => {
            Err(ReplyError::NotFound.into())
        },
        Err(Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

// TODO: replace with ProjectGroup
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
    use warp::Filter;
    use warp::http::StatusCode;

    use crate::test_util::credential_cache;

    #[sqlx::test(
        fixtures("fetch"),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        connection: PoolConnection<Postgres>,
    ) {
        let base_path = warp::any().boxed();
        let credential_cache = credential_cache(pool.clone()).await;

        let filter = warp::any()
            .clone()
            .and(crate::project_group::api(pool, base_path, credential_cache))
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
        let base_path = warp::any().boxed();
        let credential_cache = credential_cache(pool.clone()).await;

        let filter = warp::any()
            .clone()
            .and(crate::project_group::api(pool, base_path, credential_cache))
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups/00000000-0000-0000-0000-000000000000")
            .method("GET")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
