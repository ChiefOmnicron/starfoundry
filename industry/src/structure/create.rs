use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::ToSchema;

use crate::structure::service::{create, CreateStructure};
use crate::AppState;
use crate::structure::StructureUuid;
use crate::structure::error::Result;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType};

/// Create Structure
/// 
/// - Alternative route: `/latest/structure`
/// - Alternative route: `/v1/structure`
/// 
/// ---
/// 
/// Creates a new structure
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "Structures",
    request_body = CreateStructure,
    responses(
        (
            body = CreateStructureResponse,
            description = "Id of the new structure",
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
    Json(info):   Json<CreateStructure>,
) -> Result<impl IntoResponse> {
    let id = create(
        &state.pool,
        identity.character_id,
        info,
    ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateStructureResponse {
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
pub struct CreateStructureResponse {
    id: StructureUuid,
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use serde::Deserialize;
    use sqlx::PgPool;
    use starfoundry_lib_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};
    use uuid::Uuid;

    use crate::structure::service::CreateStructure;
    use crate::structure::structure_test_routes;
    use starfoundry_lib_eve_gateway::StructurePosition;

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        #[derive(Deserialize)]
        struct ResponseId {
            id: Uuid,
        }

        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateStructure {
                    name:               "My test structure".into(),
                    system_id:          30004759.into(),
                    structure_type_id:  35834.into(),
                    rigs:               vec![46497.into()],
                    services:           vec![35892.into()],
                    structure_id:       1_000_000_000_000,
                    position:           StructurePosition { x: 0f32, y: 0f32, z: 0f32 },
                }).unwrap()
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let body: ResponseId = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();

        let entry = sqlx::query!("
                SELECT pg.*
                FROM structure pg
                WHERE pg.id = $1
            ",
                body.id,
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My test structure");
        assert_eq!(entry.services.len(), 1);
        assert_eq!(entry.rigs.len(), 1);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unsupported_media_type(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "text/plain")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateStructure {
                    name:               "My test structure".into(),
                    system_id:          30004759.into(),
                    structure_type_id:  35834.into(),
                    rigs:               vec![46497.into()],
                    services:           vec![35892.into()],
                    structure_id:       1_000_000_000_000,
                    position:           StructurePosition { x: 0f32, y: 0f32, z: 0f32 },
                }).unwrap()
            ))
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn bad_request_no_body(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn bad_request_no_name(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&serde_json::json!({
                    "system_id":         30004759,
                    "structure_type_id": 35834,
                    "rigs":              vec![46497],
                    "services":          vec![35892],
                    "structure_id":      1000000000000 as i64,
                })).unwrap()
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn bad_request_no_structure_id(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&serde_json::json!({
                    "name":              "My test structure",
                    "system_id":         30004759,
                    "structure_type_id": 35834,
                    "rigs":              vec![46497],
                    "services":          vec![35892],
                })).unwrap()
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn bad_request_structure_id_lower_than(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&serde_json::json!({
                    "name":              "My test structure",
                    "system_id":         30004759,
                    "structure_type_id": 35834,
                    "rigs":              vec![46497],
                    "services":          vec![35892],
                    "structure_id":      1337 as i64,
                })).unwrap()
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
