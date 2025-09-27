mod service;
mod update;

pub use self::service::*;
pub use self::update::*;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::project_group::ProjectGroupUuid;
use crate::AppState;
use crate::project_group::error::Result;


/// Update General Group
/// 
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}`
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}`
/// 
/// ---
/// 
/// Updates a project group
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{ProjectGroupUuid}",
    tag = "project-groups",
    request_body = UpdateProjectGroup,
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            description = "The group was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    Json(update_info):        Json<UpdateProjectGroup>,
) -> Result<impl IntoResponse> {
    update(
        &state.pool,
        project_group_uuid,
        update_info,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, HOST};
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use starfoundry_lib_eve_gateway::test::JwtTokenForTesting;
    use starfoundry_lib_types::CharacterId;

    use crate::project_group::project_group_test_routes;
    use crate::project_group::update::UpdateProjectGroup;

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
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "application/json")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name: "Update Name".into(),
                }).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let entry = sqlx::query!("
                SELECT pg.*
                FROM project_group pg
                WHERE pg.id = '00000000-0000-0000-0000-000000000001'
            ")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "Update Name");
        assert_eq!(entry.description.unwrap(), "Update Description");
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn unsupported_media_type(
        pool: PgPool,
    ) {
        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "text/plain")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name: "Update Name".into(),
                }).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn bad_request_no_body(
        pool: PgPool,
    ) {
        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "application/json")
            .method("PUT")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn bad_request_no_name(
        pool: PgPool,
    ) {
        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "application/json")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&serde_json::json!({
                    "description": "My cool description",
                })).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
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
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "application/json")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name: "Update Name".into(),
                }).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        let token = JwtTokenForTesting::new(CharacterId(2));
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "application/json")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name: "Update Name".into(),
                }).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
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
            .uri("/00000000-0000-0000-0000-000000000010")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "application/json")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name: "Update Name".into(),
                }).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
