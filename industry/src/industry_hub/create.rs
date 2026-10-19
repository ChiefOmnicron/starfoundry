use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::api_docs::{
    BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::industry_hub::IndustryHubError;
use crate::industry_hub::error::Result;
use crate::{AppState, IndustryHubUuid};

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
    identity: ExtractIdentity,
    Json(info): Json<CreateIndustryHub>,
) -> Result<impl IntoResponse> {
    let id = create(&state.postgres, identity.character_id, info).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateIndustryHubResponse {
            id,
        }),
    ))
}

pub async fn create(
    pool: &PgPool,
    character_id: CharacterId,
    info: CreateIndustryHub,
) -> Result<IndustryHubUuid> {
    info.valid()?;

    sqlx::query!(
        "
            INSERT INTO industry_hub
            (
                owner,
                name
            )
            VALUES($1, $2)
            RETURNING id
        ",
        *character_id,
        info.name,
    )
    .fetch_one(pool)
    .await
    .map(|x| IndustryHubUuid::new(x.id))
    .map_err(IndustryHubError::CreateIndustryHub)
}

#[cfg(test)]
mod create_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use super::CreateIndustryHub;
    use crate::industry_hub::error::IndustryHubError;

    #[sqlx::test]
    async fn no_name(pool: PgPool) {
        let result = super::create(
            &pool,
            CharacterId(1),
            CreateIndustryHub {
                name: String::new(),
            },
        )
        .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(IndustryHubError::ValidationError(_))));
    }

    #[sqlx::test]
    async fn happy_path(pool: PgPool) {
        let result = super::create(
            &pool,
            CharacterId(1),
            CreateIndustryHub {
                name: String::from("My cool industry hub"),
            },
        )
        .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(
            r#"
                    SELECT
                        name
                    FROM industry_hub
                    WHERE id = $1
                "#,
            *result.unwrap(),
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(entry.name, "My cool industry hub");
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool industry hub"
    })
)]
pub struct CreateIndustryHub {
    /// Name of the industry hub
    pub name: String,
}

impl CreateIndustryHub {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self
                .name
                .trim()
                .is_empty()
            {
                return Err(IndustryHubError::ValidationError(
                    "Field 'name' must be set".into(),
                ));
            }
        } else {
            return Err(IndustryHubError::ValidationError(
                "Field 'name' is too long, max length: 100".into(),
            ));
        };

        Ok(true)
    }
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
    use axum::http::StatusCode;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use http_body_util::BodyExt;
    use serde::Deserialize;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use uuid::Uuid;

    use crate::industry_hub::create::CreateIndustryHub;
    use crate::industry_hub::industry_hub_test_routes;

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        #[derive(Deserialize)]
        struct ResponseId {
            id: Uuid,
        }

        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateIndustryHub {
                    name: "My test structure group".into(),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let body: ResponseId = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();

        let entry = sqlx::query!(
            "
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

    #[sqlx::test(fixtures("base"))]
    async fn unsupported_media_type(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "text/plain")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateIndustryHub {
                    name: "My test industry hub".into(),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[sqlx::test(fixtures("base"))]
    async fn bad_request_no_body(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
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
