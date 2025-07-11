mod project_group;
mod filter;
mod service;

use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::project_group::error::Result;

pub use self::filter::*;
pub use self::project_group::*;
pub use self::service::*;
use axum::extract::{Query, State};
use crate::AppStateExtractor;
use crate::auth::ExtractIdentity;

/// /project-groups
/// 
/// Alternative route: `/latest/project-groups`
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
    path = "/",
    tag = "project-groups",
    params(ProjectGroupFilter),
    responses(
        (
            body = Vec<ProjectGroupList>,
            description = "List all groups that match the given filters",
            status = OK,
        ),
        (
            body = Vec<ProjectGroupList>,
            description = "There aren't any groups stored",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):              AppStateExtractor,
    ExtractIdentity(identity): ExtractIdentity,
    Query(filter):             Query<ProjectGroupFilter>,
) -> Result<impl IntoResponse> {
    let data = list(
        &state.pool,
        identity.character_id(),
        filter,
    ).await?;

    if data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                (),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(data),
            )
            .into_response()
        )
    }
}

/*#[cfg(test)]
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
*/
