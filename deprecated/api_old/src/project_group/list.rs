mod project_group;
mod filter;
mod service;

use sqlx::PgPool;
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};

pub use self::filter::*;
pub use self::project_group::*;
pub use self::service::*;

/// /project-groups
/// 
/// Alternative route: `/v1/project-groups`
/// 
/// ---
/// 
/// Lists all project groups the user has access to.
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_groups_fetch",
    path = "/project-groups",
    tag = "project-groups",
    params(ProjectGroupFilter),
    responses(
        (
            body = Vec<ProjectGroupList>,
            description = "List all groups that match the given filters",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn list_api(
    pool:     PgPool,
    identity: Identity,
    filter:   ProjectGroupFilter,
) -> Result<impl Reply, Rejection> {
    match list(
        &pool,
        identity.character_id(),
        filter,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[cfg(test)]
mod list_project_group_test {
    use sqlx::PgPool;
    use warp::Filter;
    use warp::http::StatusCode;

    use crate::test_util::credential_cache;

    use super::ProjectGroupList;

    #[sqlx::test(
        fixtures("list"),
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
            .path("/project-groups")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroupList> = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body.len(), 4);

        let response = warp::test::request()
            .path("/project-groups?name=Filter")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroupList> = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body.len(), 2);

        let response = warp::test::request()
            .path("/project-groups?name=SomeGibberish")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroupList> = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body.len(), 0);
    }
}
