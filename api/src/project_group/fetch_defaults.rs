use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectGroupDefault, ProjectGroupService, ProjectGroupUuid};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use super::ProjectGroupUuidPath;

/// /project-groups/{projectGroupUuid}/defaults
/// 
/// Alternative route: `/v1/project-groups/{projectGroupUuid}/defaults`
/// 
/// ---
/// 
/// Fetches the defaults configured by the project group
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_groups_fetch_default",
    path = "/project-groups/{projectGroupUuid}/defaults",
    tag = "project-groups",
    params(
        ProjectGroupUuidPath,
    ),
    responses(
        (
            body = ProjectGroupDefault,
            description = "Defaults of the group",
            status = OK,
        ),
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn fetch_defaults(
    pool:               PgPool,
    identity:           Identity,
    project_group_uuid: ProjectGroupUuid,
) -> Result<impl Reply, Rejection> {
    let project_group = ProjectGroupService::new(project_group_uuid);

    match project_group.fetch_defaults(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(starfoundry_libs_projects::Error::ProjectGroupNotFound(_)) => {
            Err(ReplyError::NotFound.into())
        },
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
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
        pool: PgPool,
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
