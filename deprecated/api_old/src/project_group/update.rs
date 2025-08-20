mod service;
mod update;

use sqlx::PgPool;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project_group::{ProjectGroupUuid, ProjectGroupUuidPath};
use crate::project_group::error::Error;

pub use self::service::*;
pub use self::update::*;

/// /project-groups/{projectGroupUuid}
/// 
/// Alternative route: `/v1/project-groups/{projectGroupUuid}`
/// 
/// ---
/// 
/// Updates a project group
/// 
/// ## Security
/// - authenticated
/// - project_group: write
/// 
#[utoipa::path(
    put,
    operation_id = "project_groups_update",
    path = "/project-groups/{projectGroupUuid}",
    tag = "project-groups",
    request_body = UpdateProjectGroup,
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        (
            description = "The group was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
)]
pub async fn update_api(
    pool:               PgPool,
    _:                  Identity,
    project_group_uuid: ProjectGroupUuid,
    update_info:        UpdateProjectGroup,
) -> Result<impl Reply, Rejection> {
    match update(
        &pool,
        project_group_uuid,
        update_info,
    ).await {
        Ok(x) => {
            let response = warp::reply::with_status(
                warp::reply::json(&x),
                StatusCode::NO_CONTENT,
            );
            Ok(response)
        },
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
mod update_project_group_test {
    use serde_json::json;
    use sqlx::PgPool;
    use warp::Filter;
    use warp::http::StatusCode;

    use crate::test_util::credential_cache;

    #[sqlx::test(
        fixtures("update"),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn no_body(
        pool: PgPool,
    ) {
        let base_path = warp::any().boxed();
        let credential_cache = credential_cache(pool.clone()).await;

        let filter = warp::any()
            .clone()
            .and(crate::project_group::api(pool, base_path, credential_cache))
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups/00000000-0000-0000-0000-000000000001")
            .method("PUT")
            .json(&json!({}))
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(
        fixtures("update"),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn missing_name(
        pool: PgPool,
    ) {
        let base_path = warp::any().boxed();
        let credential_cache = credential_cache(pool.clone()).await;

        let filter = warp::any()
            .clone()
            .and(crate::project_group::api(pool, base_path, credential_cache))
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups/00000000-0000-0000-0000-000000000001")
            .method("PUT")
            .json(&json!({
                "description": "My cool description"
            }))
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(
        fixtures("update"),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        connection: PoolConnection<Postgres>,
    ) {
        let base_path = warp::any().boxed();
        let credential_cache = credential_cache(pool.clone()).await;

        let filter = warp::any()
            .clone()
            .and(crate::project_group::api(pool.clone(), base_path, credential_cache))
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups/00000000-0000-0000-0000-000000000001")
            .method("PUT")
            .json(&json!({
                "name": "My shared projects",
                "description": "My cool description"
            }))
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let entry = sqlx::query!(
                "SELECT * FROM project_group WHERE id = '00000000-0000-0000-0000-000000000001'",
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My shared projects");
        assert_eq!(entry.description.unwrap(), "My cool description");
    }
}
