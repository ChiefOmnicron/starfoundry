mod service;

use self::service::*;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::ExtractIdentity;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::project_group::error::Result;
use crate::project_group::list_members::ProjectGroupMember;
use crate::project_group::ProjectGroupUuid;

/// Fetch Members Self
/// 
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}/members/self`
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}/members/self`
/// 
/// Fetches yourself
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}/members/self",
    tag = "project-groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = ProjectGroupMember,
            description = "Your identity within the project",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
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
    let data = fetch_members_self(
            &state.pool,
            &identity.gateway_client()?,
            identity.character_info.character_id,
            project_group_uuid,
        )
        .await?;

    Ok(
        (
            StatusCode::OK,
            Json(data),
        )
        .into_response()
    )
}


#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::{AUTHORIZATION, HOST};
    use axum::http::StatusCode;
    use sqlx::PgPool;
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
            .uri("/00000000-0000-0000-0000-000000000001/members/self")
            .method("GET")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
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
            .uri("/00000000-0000-0000-0000-000000000001/members/self")
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
            .uri("/00000000-0000-0000-0000-000000000005/members/self")
            .method("GET")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
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
            .uri("/00000000-0000-0000-0000-000000000000/members/self")
            .method("GET")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
