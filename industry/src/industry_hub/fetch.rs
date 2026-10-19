use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::industry_hub::error::Result;
use crate::industry_hub::{
    IndustryHub, IndustryHubError, IndustryHubShare, IndustryHubShareType
};
use crate::structure::list::{StructureFilter, list};
use crate::{AppState, IndustryHubUuid, eve_gateway_api_client};

/// Fetch Industry Hub
///
/// - Alternative route: `/latest/industry-hub/{IndustryHubUuid}`
/// - Alternative route: `/v1/industry-hub/{IndustryHubUuid}`
///
/// ---
///
/// Fetches information about a industry hub
///
/// ## Security
/// - authenticated
#[utoipa::path(
    get,
    path = "/{IndustryHubUuid}",
    tag = "Industry Hubs",
    params(
        IndustryHubUuid,
    ),
    responses(
        (
            body = IndustryHub,
            description = "Information about the industry hub",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
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
) -> Result<impl IntoResponse> {
    let entry = fetch(
        &state.postgres,
        &eve_gateway_api_client()?,
        identity.character_id,
        industry_hub_uuid,
    )
    .await?;

    if let Some(x) = entry {
        Ok((StatusCode::OK, Json(x)).into_response())
    } else {
        Ok((StatusCode::NO_CONTENT, ()).into_response())
    }
}

pub async fn fetch(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id: CharacterId,
    industry_hub_uuid: IndustryHubUuid,
) -> Result<Option<IndustryHub>> {
    let industry_hub = sqlx::query!(
        r#"
            SELECT
                id,
                name,
                description
            FROM industry_hub
            WHERE
                owner = $1 AND
                id = $2
        "#,
        *character_id,
        *industry_hub_uuid,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| IndustryHubError::FetchIndustryHub(e, industry_hub_uuid))?;

    let industry_hub = if let Some(x) = industry_hub {
        x
    } else {
        tracing::debug!("Couldn't find structure group {}", industry_hub_uuid);
        return Ok(None);
    };

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
        IndustryHubError::FetchIndustryHub(
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

    let structures = list(
        pool,
        eve_gateway_api_client,
        character_id,
        StructureFilter {
            structure_ids: Some(structure_ids),
            ..Default::default()
        },
    )
    .await?;

    let industry_hub = IndustryHub {
        id:          industry_hub
            .id
            .into(),
        name:        industry_hub.name,
        structures:  structures,
        shares:      shares,
        description: industry_hub.description,
    };

    Ok(Some(industry_hub))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::HOST;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    use crate::eve_gateway_api_client;
    use crate::industry_hub::industry_hub_test_routes;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn direct_happy_path(pool: PgPool) {
        let response = super::fetch(
            &pool,
            &eve_gateway_api_client().unwrap(),
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-100000000001")
                .unwrap()
                .into(),
        )
        .await
        .unwrap();

        let response = response.unwrap();
        assert_eq!(response.name, "IndustryHubA".to_string());
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn no_entry_with_default_uuid(pool: PgPool) {
        let response = super::fetch(
            &pool,
            &eve_gateway_api_client().unwrap(),
            CharacterId(1),
            Uuid::default().into(),
        )
        .await;

        assert!(
            response
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000002")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000000")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
