use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_eve_gateway::structure::StructurePosition;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::{CharacterId, SystemId, TypeId};
use utoipa::ToSchema;

use crate::api_docs::{
    BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::structure::StructureError;
use crate::structure::error::Result;
use crate::{AppState, StructureUuid};

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
    identity: ExtractIdentity,
    Json(info): Json<CreateStructure>,
) -> Result<impl IntoResponse> {
    let id = create(&state.postgres, identity.character_id, info).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateStructureResponse {
            id,
        }),
    ))
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

pub async fn create(
    pool: &PgPool,
    character_id: CharacterId,
    info: CreateStructure,
) -> Result<StructureUuid> {
    info.valid()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(StructureError::TransactionError)?;

    let structure_id = sqlx::query!(
        "
            INSERT INTO structure
            (
                owner,
                type_id,
                rigs,
                services,
                name,
                system_id,
                structure_id,
                x,
                y,
                z
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
        ",
        *character_id,
        *info.structure_type_id,
        &info.rigs as _,
        &info.services as _,
        info.name,
        *info.system_id,
        info.structure_id,
        info.position.x,
        info.position.y,
        info.position.z,
    )
    .fetch_one(&mut *transaction)
    .await
    .map(|x| StructureUuid::new(x.id))
    .map_err(StructureError::CreateStructure)?;

    transaction
        .commit()
        .await
        .map_err(StructureError::TransactionError)?;

    Ok(structure_id)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "1DQ1-A - 1-st Imperial Palace",
        "rigs": [
            37275
        ],
        "security": "NULLSEC",
        "services": [
            35894
        ],
        "structure_id": 1003520240,
        "structure_type_id": 35834,
        "system_id": 30004759
    })
)]
pub struct CreateStructure {
    /// Name of the structure
    pub name:      String,
    /// Location of the structure
    pub system_id: SystemId,

    /// Type of structure
    pub structure_type_id: TypeId,
    /// List of all rigs that are in the structure
    pub rigs:              Vec<TypeId>,
    /// Id of the structure in-game
    pub services:          Vec<TypeId>,
    /// Position of the structure in the galaxy
    pub position:          StructurePosition,

    /// EVE Id of the structure
    pub structure_id: i64,
}

impl CreateStructure {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self
                .name
                .trim()
                .is_empty()
            {
                return Err(StructureError::ValidationError(
                    "Field 'name' must be set".into(),
                ));
            }
        } else {
            return Err(StructureError::ValidationError(
                "Field 'name' is too long, max length: 100".into(),
            ));
        };

        if self.structure_id < 1000000000000 {
            return Err(StructureError::ValidationError("Field 'structure_id' must be equal or larger than 1_000_000_000_000".into()));
        };

        Ok(true)
    }
}

#[cfg(test)]
mod create_project_group_test {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use http_body_util::BodyExt;
    use serde::Deserialize;
    use sqlx::PgPool;
    use starfoundry_eve_gateway::structure::{
        StructurePosition, StructureType
    };
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    use super::CreateStructure;
    use crate::structure::error::StructureError;
    use crate::structure::structure_test_routes;

    #[sqlx::test()]
    async fn no_name(pool: PgPool) {
        let result = super::create(
            &pool,
            CharacterId(1),
            CreateStructure {
                name:              String::new(),
                system_id:         1337.into(),
                structure_type_id: StructureType::Tatara.into(),
                rigs:              Vec::new(),
                services:          Vec::new(),
                structure_id:      1_000_000_000_000,
                position:          StructurePosition {
                    x: 0.0f32,
                    y: 0.0f32,
                    z: 0.0f32,
                },
            },
        )
        .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(StructureError::ValidationError(_))));
    }

    #[sqlx::test()]
    async fn structure_id_too_low(pool: PgPool) {
        let result = super::create(
            &pool,
            CharacterId(1),
            CreateStructure {
                name:              String::from("My cool structure"),
                system_id:         1337.into(),
                structure_type_id: StructureType::Tatara.into(),
                rigs:              Vec::new(),
                services:          Vec::new(),
                structure_id:      100_000_000_000,
                position:          StructurePosition {
                    x: 0.0f32,
                    y: 0.0f32,
                    z: 0.0f32,
                },
            },
        )
        .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(StructureError::ValidationError(_))));
    }

    #[sqlx::test()]
    async fn direct_happy_path(pool: PgPool) {
        let result = super::create(
            &pool,
            CharacterId(1),
            CreateStructure {
                name:              String::from("My cool structure"),
                system_id:         1337.into(),
                structure_type_id: StructureType::Tatara.into(),
                rigs:              Vec::new(),
                services:          Vec::new(),
                structure_id:      1_100_000_000_000,
                position:          StructurePosition {
                    x: 0.0f32,
                    y: 0.0f32,
                    z: 0.0f32,
                },
            },
        )
        .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(
            r#"
                    SELECT
                        name,
                        type_id,
                        rigs,
                        services,
                        structure_id
                    FROM structure WHERE id = $1
                "#,
            *result.unwrap(),
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(entry.name, "My cool structure");
        assert_eq!(entry.structure_id, 1_100_000_000_000);
    }

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
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateStructure {
                    name:              "My test structure".into(),
                    system_id:         30004759.into(),
                    structure_type_id: 35834.into(),
                    rigs:              vec![46497.into()],
                    services:          vec![35892.into()],
                    structure_id:      1_000_000_000_000,
                    position:          StructurePosition {
                        x: 0.0f32,
                        y: 0.0f32,
                        z: 0.0f32,
                    },
                })
                .unwrap(),
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
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
        assert_eq!(
            entry
                .services
                .len(),
            1
        );
        assert_eq!(
            entry
                .rigs
                .len(),
            1
        );
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
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateStructure {
                    name:              "My test structure".into(),
                    system_id:         30004759.into(),
                    structure_type_id: 35834.into(),
                    rigs:              vec![46497.into()],
                    services:          vec![35892.into()],
                    structure_id:      1_000_000_000_000,
                    position:          StructurePosition {
                        x: 0.0f32,
                        y: 0.0f32,
                        z: 0.0f32,
                    },
                })
                .unwrap(),
            ))
            .unwrap();
        let response = structure_test_routes(pool, request).await;
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
            .method("POST")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("base"))]
    async fn bad_request_no_name(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
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
                }))
                .unwrap(),
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[sqlx::test(fixtures("base"))]
    async fn bad_request_no_structure_id(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
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
                }))
                .unwrap(),
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[sqlx::test(fixtures("base"))]
    async fn bad_request_structure_id_lower_than(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
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
                }))
                .unwrap(),
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
