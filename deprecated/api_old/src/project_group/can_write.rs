use sqlx::PgPool;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, NoContent, Unauthorized};
use crate::project_group::{ProjectGroupUuid, ProjectGroupUuidPath};
use crate::project_group::error::Error;
use crate::project_group::permission::{assert_write_access, ProjectGroupPermissionCode};

/// /project-groups/{projectGroupUuid}/can-write
/// 
/// Alternative route: `/v1/project-groups/{projectGroupUuid}/can-write`
/// 
/// ---
/// 
/// Returns a 2xx status code if the requesting character is allowed to write
/// the group, otherwise a 4xx status code
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    get,
    operation_id = "project_groups_can_write",
    path = "/project-groups/{projectGroupUuid}/can-write",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        NoContent,
        Forbidden,
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn can_write_api(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    match assert_write_access(
        &pool,
        project_group_uuid,
        identity.character_id(),
        ProjectGroupPermissionCode::WriteGroup,
    ).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Err(Error::NotFound(_)) |
        Err(Error::FetchGroupPermissions(_, _)) |
        Err(Error::Forbidden(_, _)) => Err(ReplyError::Forbidden.into()),
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[cfg(test)]
mod can_write_project_group_test {
    use sqlx::PgPool;
    use warp::Filter;
    use warp::http::StatusCode;

    use crate::test_util::credential_cache;

    // TODO: add unhappy path
    #[sqlx::test(
        fixtures("can_write"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn does_not_exist(
        pool: PgPool,
    ) {
        let base_path = warp::any().boxed();
        let credential_cache = credential_cache(pool.clone()).await;

        let filter = warp::any()
            .clone()
            .and(crate::project_group::api(pool, base_path, credential_cache))
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups/00000000-0000-0000-0000-000000000000/can-write")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(
        fixtures("can_write"),
        migrator = "crate::test_util::MIGRATOR"
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
            .path("/project-groups/00000000-0000-0000-0000-000000000001/can-write")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
