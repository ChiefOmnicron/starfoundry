use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::api_docs::{
    BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::structure::StructureError;
use crate::structure::error::Result;
use crate::{AppState, StructureUuid};

/// Create Structure
///
/// - Alternative route: `/latest/structure/{StructureUuid}`
/// - Alternative route: `/v1/structure/{StructureUuid}`
///
/// ---
///
/// Creates a new structure
///
/// ## Security
/// - authenticated
#[utoipa::path(
    put,
    path = "/{StructureUuid}",
    tag = "Structures",
    request_body = UpdateStructure,
    responses(
        (
            description = "The structure was updated",
            status = NO_CONTENT,
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
    Path(structure_uuid): Path<StructureUuid>,
    Json(data): Json<UpdateStructure>,
) -> Result<impl IntoResponse> {
    update(&state.postgres, structure_uuid, data).await?;

    Ok((StatusCode::NO_CONTENT, ()))
}

pub async fn update(
    pool: &PgPool,
    structure_id: StructureUuid,
    data: UpdateStructure,
) -> Result<StructureUuid> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(StructureError::TransactionError)?;

    sqlx::query!(
        "
            DELETE FROM structure_tax
            WHERE structure_id = $1
        ",
        *structure_id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| StructureError::UpdateStructure(e, structure_id))?;

    let mut type_ids = Vec::new();
    let mut taxes = Vec::new();
    data.taxes
        .into_iter()
        .for_each(|(type_id, tax)| {
            type_ids.push(*type_id);
            taxes.push(tax);
        });

    sqlx::query!(
        "
            INSERT INTO structure_tax
            (
                structure_id,
                service_type_id,
                tax
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::REAL[]
            )
        ",
        *structure_id,
        &type_ids,
        &taxes,
    )
    .execute(&mut *transaction)
    .await
    .map_err(StructureError::CreateStructure)?;

    let structure_id = sqlx::query!(
        "
            UPDATE structure
            SET
                rigs = $2,
                services = $3
            WHERE id = $1
        ",
        *structure_id,
        &data.rigs as _,
        &data.services as _,
    )
    .execute(&mut *transaction)
    .await
    .map(|_| structure_id)
    .map_err(|e| StructureError::UpdateStructure(e, structure_id))?;

    transaction
        .commit()
        .await
        .map_err(StructureError::TransactionError)?;

    Ok(structure_id)
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "rigs": [
            37275
        ],
        "services": [
            35894
        ]
    })
)]
pub struct UpdateStructure {
    /// List of all rigs that are in the structure
    pub rigs:     Vec<TypeId>,
    /// Id of the structure in-game
    pub services: Vec<TypeId>,
    /// Taxes based on service type id
    pub taxes:    HashMap<TypeId, f32>,
}

#[cfg(test)]
mod update_structure_test {
    use std::collections::HashMap;
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use uuid::Uuid;

    use super::UpdateStructure;
    use crate::structure::structure_test_routes;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base")))]
    async fn direct_happy_path(pool: PgPool) {
        let result = super::update(
            &pool,
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
            UpdateStructure {
                rigs:     vec![1, 2, 3]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
                services: vec![1, 2, 3, 4, 5]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
                taxes:    HashMap::new(),
            },
        )
        .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(r#"
                    SELECT
                        name,
                        type_id,
                        rigs,
                        services,
                        structure_id
                    FROM structure WHERE id = '00000000-0000-0000-0000-000000000001'
                "#,
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(
            entry
                .rigs
                .len(),
            3
        );
        assert_eq!(entry.rigs, vec![1, 2, 3]);
        assert_eq!(
            entry
                .services
                .len(),
            5
        );
        assert_eq!(entry.services, vec![1, 2, 3, 4, 5]);
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateStructure {
                    rigs:     vec![1.into(), 2.into(), 3.into()],
                    services: vec![
                        1.into(),
                        2.into(),
                        3.into(),
                        4.into(),
                        5.into(),
                    ],
                    taxes:    HashMap::new(),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let entry = sqlx::query!(
            "
                SELECT pg.*
                FROM structure pg
                WHERE pg.id = '00000000-0000-0000-0000-000000000001'
            "
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(
            entry
                .rigs
                .len(),
            3
        );
        assert_eq!(entry.rigs, vec![1, 2, 3]);
        assert_eq!(
            entry
                .services
                .len(),
            5
        );
        assert_eq!(entry.services, vec![1, 2, 3, 4, 5]);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unsupported_media_type(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "text/plain")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateStructure {
                    rigs:     vec![1.into()],
                    services: vec![1.into()],
                    taxes:    HashMap::new(),
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
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
