mod service;

use sqlx::PgPool;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, NoContent, NotFound, Unauthorized};
use crate::project_group::error::Error;
use crate::project_group::ProjectGroupUuid;

use super::ProjectGroupUuidPath;

pub use self::service::*;

/// /project-groups/{projectGroupUuid}
/// 
/// Alternative route: `/v1/project-groups/{projectGroupUuid}`
/// 
/// ---
/// 
/// Deletes the group
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    delete,
    operation_id = "project_groups_delete",
    path = "/project-groups/{projectGroupUuid}",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        NoContent,
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn delete_api(
    pool:               PgPool,
    _:                  Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {

    match delete(
        &pool,
        project_group_uuid,
    ).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Err(Error::NotFound(_)) => {
            Err(ReplyError::NotFound.into())
        },
        Err(Error::FetchGroupPermissions(_, _)) |
        Err(Error::Forbidden(_, _)) => Err(ReplyError::Forbidden.into()),
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[cfg(test)]
mod delete_project_group_test {
    use sqlx::PgPool;
    use warp::Filter;
    use warp::http::StatusCode;

    use crate::test_util::credential_cache;

    #[sqlx::test(
        fixtures("delete"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn user_does_not_have_the_permission(
        pool: PgPool,
    ) {
        let base_path = warp::any().boxed();
        let credential_cache = credential_cache(pool.clone()).await;

        let filter = warp::any()
            .clone()
            .and(crate::project_group::api(pool, base_path, credential_cache))
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups/00000000-0000-0000-0000-000000000002")
            .method("DELETE")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(
        fixtures("delete"),
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
            .path("/project-groups/00000000-0000-0000-0000-000000000000")
            .method("DELETE")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[sqlx::test(
        fixtures("delete"),
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
            .path("/project-groups/00000000-0000-0000-0000-000000000001")
            .method("DELETE")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
