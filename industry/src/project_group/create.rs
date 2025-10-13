mod project_group;
mod service;

pub use self::project_group::*;
pub use self::service::*;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use starfoundry_lib_eve_gateway::ExtractIdentity;
use utoipa::ToSchema;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::project_group::error::Result;
use crate::project_group::ProjectGroupUuid;

/// Create Group
/// 
/// - Alternative route: `/latest/project-groups`
/// - Alternative route: `/v1/project-groups`
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
    path = "/",
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
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    identity:     ExtractIdentity,
    Json(info):   Json<CreateProjectGroup>,
) -> Result<impl IntoResponse> {
    let id = create(
        &state.pool,
        identity.character_id,
        info,
    ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateProjectResponse {
                id: id.into(),
            })
        )
    )
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateProjectResponse {
    id: ProjectGroupUuid,
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, HOST};
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use serde::Deserialize;
    use sqlx::PgPool;
    use starfoundry_lib_eve_gateway::test::JwtTokenForTesting;
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    use crate::project_group::create::CreateProjectGroup;
    use crate::project_group::project_group_test_routes;
    use starfoundry_lib_eve_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        #[derive(Deserialize)]
        struct ResponseId {
            id: Uuid,
        }

        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateProjectGroup {
                    description: Some("My cool description".into()),
                    name: "My shared projects".into(),
                }).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let body: ResponseId = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();

        let entry = sqlx::query!("
                SELECT pg.*
                FROM project_group pg
                JOIN project_group_member pgm ON pgm.group_id = pg.id
                JOIN project_group_default_market pgdm ON pgdm.project_group_id = pg.id
                WHERE pg.id = $1
            ",
                body.id,
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My shared projects");
        assert_eq!(entry.description.unwrap(), "My cool description");
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
            .uri("/")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "text/plain")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateProjectGroup {
                    description: Some("My cool description".into()),
                    name: "My shared projects".into(),
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
            .uri("/")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .method("POST")
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
            .uri("/")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&serde_json::json!({
                    "description": "My cool description",
                })).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
