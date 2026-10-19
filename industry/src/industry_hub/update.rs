use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::industry_hub::error::Result;
use crate::industry_hub::{IndustryHubError, IndustryHubShare};
use crate::{AppState, IndustryHubUuid};

/// Update Industry Hub
///
/// - Alternative route: `/latest/industry-hubs/{IndustryHubUuid}`
/// - Alternative route: `/v1/industry-hubs/{IndustryHubUuid}`
///
/// ---
///
/// Lists all available industry hubs
///
/// ## Security
/// - authenticated
#[utoipa::path(
    put,
    path = "/{IndustryHubUuid}",
    tag = "Industry Hubs",
    request_body = UpdateIndustryHub,
    params(
        IndustryHubUuid,
    ),
    responses(
        (
            description = "The structure group was successfully updated",
            status = NO_CONTENT
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
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(industry_hub_uuid): Path<IndustryHubUuid>,
    Json(industry_hub): Json<UpdateIndustryHub>,
) -> Result<impl IntoResponse> {
    update(
        &state.postgres,
        identity.character_id,
        industry_hub_uuid,
        industry_hub,
    )
    .await?;

    Ok((StatusCode::NO_CONTENT, Json(())).into_response())
}

pub async fn update(
    pool: &PgPool,
    character_id: CharacterId,
    industry_hub_uuid: IndustryHubUuid,
    industry_hub: UpdateIndustryHub,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(IndustryHubError::TransactionError)?;

    sqlx::query!(
        "
            UPDATE industry_hub
            SET
                name = $3,
                description = $4
            WHERE
                id = $1 AND
                owner = $2
        ",
        *industry_hub_uuid,
        *character_id,
        industry_hub.name,
        industry_hub.description,
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| IndustryHubError::UpdateIndustryHub(e, industry_hub_uuid))?;

    sqlx::query!(
        "
            DELETE FROM industry_hub_structure
            WHERE industry_hub_id = $1
        ",
        *industry_hub_uuid,
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| IndustryHubError::UpdateIndustryHub(e, industry_hub_uuid))?;

    sqlx::query!(
        "
            INSERT INTO industry_hub_structure
            (
                industry_hub_id,
                structure_id
            )
            SELECT $1, * FROM UNNEST($2::UUID[])
        ",
        *industry_hub_uuid,
        &industry_hub.structures,
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| IndustryHubError::UpdateIndustryHub(e, industry_hub_uuid))?;

    sqlx::query!(
        "
            DELETE FROM industry_hub_share
            WHERE industry_hub_id = $1
        ",
        *industry_hub_uuid,
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| IndustryHubError::UpdateIndustryHub(e, industry_hub_uuid))?;

    let share_ids = industry_hub
        .shares
        .iter()
        .map(|x| x.share_id)
        .collect::<Vec<_>>();
    let share_types = industry_hub
        .shares
        .iter()
        .map(|x| x.share_type)
        .collect::<Vec<_>>();
    let names = industry_hub
        .shares
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<_>>();
    sqlx::query!(
        "
            INSERT INTO industry_hub_share
            (
                industry_hub_id,
                share_id,
                share_type,
                name
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::SHARE_TYPE[],
                $4::VARCHAR[]
            )
        ",
        *industry_hub_uuid,
        &share_ids,
        &share_types as _,
        &names,
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| IndustryHubError::UpdateIndustryHub(e, industry_hub_uuid))?;

    transaction
        .commit()
        .await
        .map(drop)
        .map_err(IndustryHubError::TransactionError)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool structure group",
        "structures": [
            "019a462f-dd0a-7d07-9516-df377ae11395",
            "019aa90f-4e83-71b3-ad36-2844f1d2701a"
        ]
    })
)]
pub struct UpdateIndustryHub {
    pub name:       String,
    pub structures: Vec<Uuid>,
    pub shares:     Vec<IndustryHubShare>,

    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    use crate::industry_hub::industry_hub_test_routes;
    use crate::industry_hub::update::UpdateIndustryHub;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn direct_happy_path_all(pool: PgPool) {
        let result = super::update(
            &pool,
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-100000000001")
                .unwrap()
                .into(),
            UpdateIndustryHub {
                name:        "My structure group".into(),
                structures:  vec![
                    Uuid::from_str("00000000-0000-0000-0000-000000000001")
                        .unwrap(),
                ],
                shares:      Vec::new(),
                description: None,
            },
        )
        .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(
            r#"
                    SELECT
                        name
                    FROM industry_hub
                    WHERE id = '00000000-0000-0000-0000-100000000001'
                "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(entry.name, "My structure group");

        let entries = sqlx::query!(r#"
                    SELECT structure_id
                    FROM industry_hub_structure
                    WHERE industry_hub_id = '00000000-0000-0000-0000-100000000001'
                "#,
            )
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x| x.structure_id)
            .collect::<Vec<_>>();
        assert_eq!(entries.len(), 1);
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path_all(pool: PgPool) {
        let request = Request::builder()
            .header(CONTENT_TYPE, "application/json")
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("PUT")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::new(
                serde_json::to_string(&UpdateIndustryHub {
                    name:        "My cool structure group".into(),
                    structures:  Vec::new(),
                    shares:      Vec::new(),
                    description: None,
                })
                .unwrap(),
            ))
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("PUT")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .header(CONTENT_TYPE, "application/json")
            .uri("/00000000-0000-0000-0000-100000000002")
            .method("PUT")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::new(
                serde_json::to_string(&UpdateIndustryHub {
                    name:        "My cool structure group".into(),
                    structures:  Vec::new(),
                    shares:      Vec::new(),
                    description: None,
                })
                .unwrap(),
            ))
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
