use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_libs_projects::{CreateProjectGroup, ProjectGroupService};
use utoipa::ToSchema;
use uuid::Uuid;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{Identity, ReplyError};
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};

/// /project-groups
/// 
/// Alternative route: `/v1/project-groups`
/// 
/// ---
/// 
/// Creates a new project group
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    operation_id = "project_groups_create",
    path = "/project-groups",
    tag = "project-groups",
    request_body = CreateProjectGroup,
    responses(
        (
            body = CreateProjectResponse,
            description = "Id of the new project group",
            status = CREATED,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn create(
    pool:     PgPool,
    identity: Identity,
    info:     CreateProjectGroup,
) -> Result<impl Reply, Rejection> {
    match ProjectGroupService::create(
        &pool,
        identity.character_id(),
        info,
    ).await {
        Ok(x) => {
            let response = warp::reply::with_status(
                warp::reply::json(&CreateProjectResponse {
                    id: *x,
                }),
                StatusCode::CREATED,
            );
            Ok(response)
        },
        Err(starfoundry_libs_projects::Error::ValidationError(e)) => {
            tracing::error!("{e}");
            Err(ReplyError::Validation(e).into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateProjectResponse {
    id: Uuid,
}

#[cfg(test)]
mod create_project_group_test {
    use serde_json::json;
    use sqlx::PgPool;
    use warp::Filter;
    use warp::http::StatusCode;

    use crate::test_util::credential_cache;
    use crate::project_group::CreateProjectResponse;

    #[sqlx::test(migrator = "crate::test_util::MIGRATOR")]
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
            .path("/project-groups")
            .method("POST")
            .json(&json!({}))
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(migrator = "crate::test_util::MIGRATOR")]
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
            .path("/project-groups")
            .method("POST")
            .json(&json!({
                "description": "My cool description"
            }))
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(migrator = "crate::test_util::MIGRATOR")]
    async fn happy_path(
        pool: PgPool,
    ) {
        let base_path = warp::any().boxed();
        let credential_cache = credential_cache(pool.clone()).await;

        let filter = warp::any()
            .clone()
            .and(crate::project_group::api(pool.clone(), base_path, credential_cache))
            .recover(crate::rejection::handle_rejection);

        let response = warp::test::request()
            .path("/project-groups")
            .method("POST")
            .json(&json!({
                "name": "My shared projects",
                "description": "My cool description"
            }))
            .reply(&filter)
            .await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body: CreateProjectResponse = serde_json::from_slice(response.body()).unwrap();
        let entry = sqlx::query!(
                "SELECT * FROM project_group WHERE id = $1",
                body.id,
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My shared projects");
        assert_eq!(entry.description.unwrap(), "My cool description");
    }
}
