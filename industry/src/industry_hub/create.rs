use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::ToSchema;

use crate::industry_hub::service::{create, CreateIndustryHub};
use crate::AppState;
use crate::industry_hub::IndustryHubUuid;
use crate::industry_hub::error::Result;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType};

/// Create Industry Hub
/// 
/// - Alternative route: `/latest/industry-hub`
/// - Alternative route: `/v1/industry-hub`
/// 
/// ---
/// 
/// Creates a new industry hub
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "Industry Hubs",
    request_body = CreateIndustryHub,
    responses(
        (
            body = CreateIndustryHubResponse,
            description = "Id of the new structure group",
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
    Json(info):   Json<CreateIndustryHub>,
) -> Result<impl IntoResponse> {
    let id = create(
        &state.pool,
        identity.character_id,
        info,
    ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateIndustryHubResponse {
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
pub struct CreateIndustryHubResponse {
    id: IndustryHubUuid,
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

    use crate::industry_hub::industry_hub_test_routes;
    use crate::industry_hub::service::CreateIndustryHub;

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
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateIndustryHub {
                    name: "My test structure group".into(),
                }).unwrap()
            ))
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let body: ResponseId = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();

        let entry = sqlx::query!("
                SELECT *
                FROM industry_hub
                WHERE id = $1
            ",
                body.id,
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My test structure group");
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
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateIndustryHub {
                    name: "My test industry hub".into(),
                }).unwrap()
            ))
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
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
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
