use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::structure::Structure;
use crate::structure::fetch::FetchStructureQuery;
use crate::{AppState, ProjectGroupUuid, eve_gateway_api_client};

/// List Markets
///
/// - Alternative route:
///   `/latest/project-groups/{ProjectGroupUuid}/defaults/markets`
/// Alternative route: `/v1/project-groups/{ProjectGroupUuid}/defaults/markets`
///
/// ---
///
/// Fetches the defaults configured for markets
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}/defaults/markets",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = Vec<Structure>,
            description = "All markets that are configured for a project group",
            status = OK,
        ),
        (
            description = "No markets configured",
            status = NO_CONTENT,
        ),
        NotFound,
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
    Path(project_group_uuid): Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    let data = list_default_market(
        &state.postgres,
        identity.character_id,
        &eve_gateway_api_client()?,
        project_group_uuid,
    )
    .await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list_default_market(
    pool: &PgPool,
    character_id: CharacterId,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Vec<Structure>> {
    let markets = sqlx::query!(
        "
            SELECT structure_id
            FROM project_group_default_market
            WHERE project_group_id = $1
        ",
        *project_group_uuid,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ProjectGroupError::FetchGroupDefaults(e, project_group_uuid))?
    .into_iter()
    .map(|x| {
        x.structure_id
            .into()
    })
    .collect::<Vec<_>>();

    let mut structures = Vec::new();
    for market in markets {
        if let Some(x) = crate::structure::fetch::fetch(
            pool,
            eve_gateway_api_client,
            character_id,
            market,
            FetchStructureQuery::default(),
        )
        .await?
        {
            structures.push(x);
        } else {
            // silently ignore errors
            tracing::debug!("Couldn't find structure {}", market);
            continue;
        }
    }

    Ok(structures)
}

#[cfg(test)]
mod list_default_market_project_group_test {
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::HOST;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    use crate::eve_gateway_api_client;
    use crate::project_group::project_group_test_routes;
    use crate::structure::Structure;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn direct_happy_path(pool: PgPool) {
        let response = super::list_default_market(
            &pool,
            CharacterId(1),
            &eve_gateway_api_client().unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
        )
        .await
        .unwrap();

        assert_eq!(response.len(), 1);
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn default_if_entry_does_not_exist(pool: PgPool) {
        let response = super::list_default_market(
            &pool,
            CharacterId(1),
            &eve_gateway_api_client().unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000000")
                .unwrap()
                .into(),
        )
        .await
        .unwrap();

        assert_eq!(response.len(), 0);
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path_all(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/markets")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<Structure> = serde_json::from_slice(
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
    async fn happy_path_no_content(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000005/defaults/markets")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 2)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body: Vec<Structure> = serde_json::from_slice(
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
            .uri("/00000000-0000-0000-0000-000000000001/defaults/markets")
            .method("GET")
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000005/defaults/markets")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000/defaults/markets")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
