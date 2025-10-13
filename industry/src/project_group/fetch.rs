mod service;

pub use self::service::*;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::{ExtractIdentity, MtlsApiClient};

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::{api_client, AppState};
use crate::project_group::error::Result;
use crate::project_group::list::ProjectGroup;
use crate::project_group::ProjectGroupUuid;

/// Fetch Group
/// 
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}`
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}`
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
    path = "/{ProjectGroupUuid}",
    tag = "project-groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = ProjectGroup,
            description = "Information about the group",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    identity:                 ExtractIdentity,
) -> Result<impl IntoResponse> {
    let entry = fetch(
            &state.pool,
            &api_client()?,
            identity.character_id,
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

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::{AUTHORIZATION, HOST};
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use starfoundry_lib_eve_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};
    use starfoundry_lib_eve_gateway::test::JwtTokenForTesting;
    use starfoundry_lib_types::CharacterId;

    use crate::project_group::project_group_test_routes;

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000005")
            .method("GET")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn not_found(
        pool: PgPool,
    ) {
        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000")
            .method("GET")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
