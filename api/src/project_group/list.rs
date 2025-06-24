use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroup, ProjectGroupFilter, ProjectGroupService};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};

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
    params(
        (
            "name" = Option<String>,
            Query,
            description = "Fuzzy search for a name"
        ),
    ),
    responses(
        (
            body = Vec<ProjectGroup>,
            description = "List all groups that match the given filters",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn list(
    pool:     PgPool,
    identity: Identity,
    filter:   ProjectGroupFilter,
) -> Result<impl Reply, Rejection> {
    match ProjectGroupService::list(
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
    use crate::{with_identity, with_pool};
    use starfoundry_libs_projects::ProjectGroup;

    #[sqlx::test(
        fixtures("list"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let filter = warp::any()
            .clone()
            .and(with_pool(pool.clone()))
            .and(with_identity(pool.clone(), credential_cache(pool.clone()).await))
            .and(warp::path!("project-groups"))
            .and(warp::get())
            .and(warp::query())
            .and_then(super::list)
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroup> = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body.len(), 4);

        let response = warp::test::request()
            .path("/project-groups?name=Filter")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroup> = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body.len(), 2);

        let response = warp::test::request()
            .path("/project-groups?name=SomeGibberish")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroup> = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body.len(), 0);
    }
}
