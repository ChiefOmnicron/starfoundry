use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_eve_gateway::asset::Item;
use utoipa::ToSchema;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::{AppState, ProjectGroupUuid, eve_gateway_api_client};

/// List Blueprint overwrites
///
/// - Alternative route:
///   `/latest/project-groups/{ProjectGroupUuid}/defaults/blueprint-overwrites`
/// - Alternative route:
///   `/v1/project-groups/{ProjectGroupUuid}/defaults/blueprint-overwrites`
///
/// ---
///
/// Fetches the defaults configured for blueprint overwrites
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}/defaults/blueprint-overwrites",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = Vec<BlueprintOverwrite>,
            description = "All blueprint overwrites items that are configured for a project group",
            status = OK,
        ),
        (
            description = "No blueprint overwrites configured",
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
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    let data = list_default_blueprint_overwrite(
        &state.postgres,
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
pub async fn list_default_blueprint_overwrite(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Vec<BlueprintOverwrite>> {
    let entries = sqlx::query!(
        "
            SELECT
                type_id,
                material_efficiency
            FROM project_group_default_blueprint_overwrite
            WHERE project_group_id = $1
        ",
        *project_group_uuid,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        ProjectGroupError::FetchGroupDefaults(e, project_group_uuid)
    })?;

    let type_ids = entries
        .iter()
        .map(|x| {
            x.type_id
                .into()
        })
        .collect::<Vec<_>>();
    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let mut blueprint_overwrites = Vec::new();
    for entry in entries {
        if let Some(x) = items.get(
            &entry
                .type_id
                .into(),
        ) {
            blueprint_overwrites.push(BlueprintOverwrite {
                material_efficiency: entry.material_efficiency,
                item:                x.clone(),
            })
        } else {
            // silently ignore errors
            tracing::debug!("Couldn't find item {}", entry.type_id);
            continue
        }
    }

    Ok(blueprint_overwrites)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "material_efficiency": 10,
        "item": {
            "base_price": null,
            "category": {
                "category_id": 0,
                "name": "#System"
            },
            "group": {
                "group_id": 0,
                "category_id": 0,
                "name": "#System"
            },
            "meta_group_id": null,
            "name": "Ragnarok",
            "repackaged": 10000000,
            "type_id": 23773,
            "volume": 100000000
        }
    })
)]
pub struct BlueprintOverwrite {
    pub material_efficiency: i32,
    pub item:                Item,
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
    use uuid::Uuid;

    use crate::eve_gateway_api_client;
    use crate::project_group::list_default_blueprint_overwrites::BlueprintOverwrite;
    use crate::project_group::project_group_test_routes;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn direct_happy_path(pool: PgPool) {
        let response = super::list_default_blueprint_overwrite(
            &pool,
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
        let response = super::list_default_blueprint_overwrite(
            &pool,
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
            .uri("/00000000-0000-0000-0000-000000000001/defaults/blueprint-overwrites")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<BlueprintOverwrite> = serde_json::from_slice(
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
            .uri("/00000000-0000-0000-0000-000000000005/defaults/blueprint-overwrites")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 2)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body: Vec<BlueprintOverwrite> = serde_json::from_slice(
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
            .uri("/00000000-0000-0000-0000-000000000001/defaults/blueprint-overwrites")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000005/defaults/blueprint-overwrites")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000/defaults/blueprint-overwrites")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
