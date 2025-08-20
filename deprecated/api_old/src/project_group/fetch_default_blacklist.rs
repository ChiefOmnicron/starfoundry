mod service;

use sqlx::PgPool;
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::item::Item;
use crate::project_group::error::Error;
use crate::project_group::ProjectGroupUuid;

use super::ProjectGroupUuidPath;

pub use self::service::*;

/// /project-groups/{projectGroupUuid}/defaults/blacklist
/// 
/// Alternative route: `/v1/project-groups/{projectGroupUuid}/defaults/blacklist`
/// 
/// ---
/// 
/// Fetches the defaults configured for blacklist
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_groups_fetch_default_blacklist",
    path = "/project-groups/{projectGroupUuid}/defaults/blacklist",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        (
            body = Vec<Item>,
            description = "All blacklist that are configured for a project group",
            status = OK,
        ),
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn fetch_default_blacklist_api(
    pool:               PgPool,
    _:                  Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    match fetch_default_blacklist(
        &pool,
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

#[cfg(test)]
mod fetch_defaults_project_group_test {
    use sqlx::PgPool;
    use warp::Filter;
    use warp::http::StatusCode;

    use crate::test_util::credential_cache;

    #[sqlx::test(
        fixtures("fetch", "fetch_default"),
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
            .path("/project-groups/00000000-0000-0000-0000-000000000001/defaults")
            .method("GET")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(
        fixtures("fetch", "fetch_default"),
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
            .path("/project-groups/00000000-0000-0000-0000-000000000000/defaults")
            .method("GET")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
