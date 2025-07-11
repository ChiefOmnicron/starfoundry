mod project_group;
mod service;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppStateExtractor;
use crate::auth::ExtractIdentity;
use crate::project_group::error::Result;
use crate::project_group::ProjectGroupUuid;

pub use self::service::*;
use crate::project_group::fetch::project_group::ProjectGroupFetch;

/// /project-groups/{projectGroupUuid}
/// 
/// Alternative route: `/latest/project-groups/{projectGroupUuid}`
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
    path = "/{projectGroupUuid}",
    tag = "project-groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = ProjectGroupFetch,
            description = "Information about the group",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):              AppStateExtractor,
    ExtractIdentity(identity): ExtractIdentity,
    Path(project_group_uuid):  Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    let entry = fetch(
            &state.pool,
            identity.character_id(),
            project_group_uuid
        )
        .await?;

    if let Some(x) = entry {
        Ok(
            (
                StatusCode::OK,
                Json(x)
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::NO_CONTENT,
                ()
            )
            .into_response()
        )
    }
}

/*#[cfg(test)]
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
*/
