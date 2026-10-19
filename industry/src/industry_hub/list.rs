use std::fmt;

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};
use utoipa::IntoParams;
use uuid::Uuid;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::industry_hub::error::Result;
use crate::industry_hub::{
    IndustryHub, IndustryHubError, IndustryHubShare, IndustryHubShareType
};
use crate::structure::list::StructureFilter;
use crate::{AppState, eve_gateway_api_client};

/// List Industry Hubs
///
/// - Alternative route: `/latest/industry-hubs`
/// - Alternative route: `/v1/industry-hubs`
///
/// ---
///
/// Lists all available industry hubs the logged in user has access to.
/// This includes shared industry hubs.
///
/// ## Security
/// - authenticated
/// - read
#[utoipa::path(
    get,
    path = "/",
    tag = "Industry Hubs",
    params(
        IndustryHubFilter
    ),
    responses(
        (
            body = Vec<IndustryHub>,
            description = "All industry hubs the user has access to",
            status = OK,
        ),
        (
            description = "No results for your request",
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
    Query(filter): Query<IndustryHubFilter>,
) -> Result<impl IntoResponse> {
    let data = list(
        &state.postgres,
        &eve_gateway_api_client()?,
        identity.character_id,
        identity.corporation_id,
        identity.alliance_id,
        filter,
    )
    .await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id: CharacterId,
    corporation_id: CorporationId,
    alliance_id: Option<AllianceId>,
    filter: IndustryHubFilter,
) -> Result<Vec<IndustryHub>> {
    struct TmpIndustryHub {
        id:          Uuid,
        name:        String,
        description: Option<String>,
    }

    let shared = match filter.shared {
        Some(true) => true,
        Some(false) | None => false,
    };

    let industry_hubs = if shared {
        sqlx::query!(r#"
                SELECT
                    DISTINCT(ih.id),
                    ih.name,
                    ih.description
                FROM industry_hub ih
                LEFT JOIN industry_hub_structure ihs ON ihs.industry_hub_id = ih.id
                LEFT JOIN structure s ON s.id = ihs.structure_id
                LEFT JOIN industry_hub_share ihsa ON ihsa.industry_hub_id = ih.id
                WHERE
                    NOT (LOWER(ih.name) LIKE '%' || LOWER($4) || '%') IS FALSE AND
                    NOT (s.type_id = $5) IS FALSE AND
                    NOT (s.system_id = $6) IS FALSE AND
                    NOT ($5::INTEGER IS NULL OR $7::INTEGER = ANY(s.services)) IS FALSE AND
                    NOT ($6::INTEGER IS NULL OR $8::INTEGER = ANY(s.rigs)) IS FALSE AND
                    (
                        ihsa.share_id = $1 OR
                        ihsa.share_id = $2 OR
                        ihsa.share_id = $3
                    ) AND
                    ih.owner != $1
                ORDER BY ih.name
            "#,
                *character_id,
                *corporation_id,
                alliance_id.map(|x| *x).unwrap_or(0),
                filter.name,
                filter.structure_type_id,
                filter.system_id,
                filter.service_id,
                filter.rig_id,
            )
            .fetch_all(pool)
            .await
            .map_err(IndustryHubError::ListIndustryHubs)?
            .into_iter()
            .map(|x| TmpIndustryHub {
                id:          x.id,
                name:        x.name,
                description: x.description,
            })
            .collect::<Vec<_>>()
    } else {
        sqlx::query!(r#"
                SELECT
                    DISTINCT(ih.id),
                    ih.name,
                    ih.description
                FROM industry_hub ih
                LEFT JOIN industry_hub_structure sgs ON sgs.industry_hub_id = ih.id
                LEFT JOIN structure s ON s.id = sgs.structure_id
                WHERE
                    NOT (LOWER(ih.name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                    NOT (s.type_id = $3) IS FALSE AND
                    NOT (s.system_id = $4) IS FALSE AND
                    NOT ($5::INTEGER IS NULL OR $5::INTEGER = ANY(s.services)) IS FALSE AND
                    NOT ($6::INTEGER IS NULL OR $6::INTEGER = ANY(s.rigs)) IS FALSE AND
                    ih.owner = $1
                ORDER BY ih.name
            "#,
                *character_id,
                filter.name,
                filter.structure_type_id,
                filter.system_id,
                filter.service_id,
                filter.rig_id,
            )
            .fetch_all(pool)
            .await
            .map_err(IndustryHubError::ListIndustryHubs)?
            .into_iter()
            .map(|x| TmpIndustryHub {
                id:          x.id,
                name:        x.name,
                description: x.description,
            })
            .collect::<Vec<_>>()
    };

    let mut result = Vec::new();
    for industry_hub in industry_hubs {
        let structure_ids = sqlx::query!(
            "
                SELECT
                    structure_id
                FROM industry_hub_structure
                WHERE industry_hub_id = $1
            ",
            industry_hub.id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            IndustryHubError::FetchIndustryHubStructures(
                e,
                industry_hub
                    .id
                    .into(),
            )
        })?
        .into_iter()
        .map(|x| {
            x.structure_id
                .into()
        })
        .collect();

        let structures = if shared {
            crate::structure::list::list_shared(
                pool,
                eve_gateway_api_client,
                StructureFilter {
                    structure_ids: Some(structure_ids),
                    ..Default::default()
                },
            )
            .await?
        } else {
            crate::structure::list::list(
                pool,
                eve_gateway_api_client,
                character_id,
                StructureFilter {
                    structure_ids: Some(structure_ids),
                    ..Default::default()
                },
            )
            .await?
        };

        let shares = sqlx::query!(
            r#"
                SELECT
                    share_id,
                    share_type AS "share_type!: IndustryHubShareType",
                    name
                FROM industry_hub_share
                WHERE industry_hub_id = $1
            "#,
            industry_hub.id,
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|x| {
            IndustryHubShare {
                name:       x.name,
                share_id:   x.share_id,
                share_type: x.share_type,
            }
        })
        .collect::<Vec<_>>();

        result.push(IndustryHub {
            id:          industry_hub
                .id
                .into(),
            name:        industry_hub.name,
            structures:  structures,
            shares:      shares,
            description: industry_hub.description,
        });
    }

    Ok(result)
}

#[derive(Debug, Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct IndustryHubFilter {
    #[serde(default)]
    #[param(
        example = json!("My cool structure"),
        required = false,
    )]
    pub name: Option<String>,

    #[serde(default)]
    #[param(
        example = json!("35827"),
        required = false,
    )]
    pub structure_type_id: Option<i32>,

    #[serde(default)]
    #[param(
        example = json!("30004759"),
        required = false,
    )]
    pub system_id: Option<i32>,

    /// [TypeId] of a structure service
    #[serde(default)]
    #[param(
        example = json!("35892"),
        required = false,
    )]
    pub service_id: Option<i32>,

    /// [TypeId] of a structure rig
    #[serde(default)]
    #[param(
        example = json!("46497"),
        required = false,
    )]
    pub rig_id: Option<i32>,

    /// Wether to only return shared industry hubs or not
    #[serde(default)]
    pub shared: Option<bool>,
}

impl fmt::Display for IndustryHubFilter {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod list_structure_test {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::HOST;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};

    use crate::industry_hub::list::IndustryHubFilter;
    use crate::industry_hub::{IndustryHub, industry_hub_test_routes};
    use crate::test_util::EveGatewayTestApiClient;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base")))]
    async fn happy_path(pool: PgPool) {
        let gateway_client = EveGatewayTestApiClient::new();
        let result = super::list(
            &pool,
            &gateway_client,
            CharacterId(1),
            CorporationId(1),
            Some(AllianceId(0)),
            IndustryHubFilter::default(),
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(
            result
                .unwrap()
                .len(),
            1
        );

        let result = super::list(
            &pool,
            &gateway_client,
            CharacterId(1),
            CorporationId(1),
            Some(AllianceId(0)),
            IndustryHubFilter {
                name: Some(String::from("IndustryHubA")),
                ..Default::default()
            },
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(
            result
                .unwrap()
                .len(),
            1
        );

        let result = super::list(
            &pool,
            &gateway_client,
            CharacterId(1),
            CorporationId(1),
            Some(AllianceId(0)),
            IndustryHubFilter {
                name: Some(String::from("SomeGibberish")),
                ..Default::default()
            },
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(
            result
                .unwrap()
                .len(),
            0
        );
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path_all(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<IndustryHub> = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();
        assert_eq!(body.len(), 1);
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path_filter(pool: PgPool) {
        // Filter
        let request = Request::builder()
            .uri("/?name=IndustryHubA")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<IndustryHub> = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();
        assert_eq!(body.len(), 1);
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path_empty(pool: PgPool) {
        // Empty
        let request = Request::builder()
            .uri("/?name=SomeGibberish")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body: Vec<IndustryHub> = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();
        assert_eq!(body.len(), 0);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
