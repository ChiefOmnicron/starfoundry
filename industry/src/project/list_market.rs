use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_eve_gateway::item::EveGatewayApiClientItem;
use starfoundry_gateway::ExtractIdentity;

use crate::api_docs::{
    BadRequest, InternalServerError, NotFound, Unauthorized
};
use crate::project::error::{ProjectError, Result};
use crate::project::fetch::ProjectMarket;
use crate::{
    AppState, ProjectUuid, eve_gateway_api_client, sort_by_market_group_flat
};

/// List Market
///
/// - Alternative route: `/latest/projects/{ProjectUuid}/market`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market`
///
/// ---
///
/// Lists all materials that need to be bought
///
/// ## Security
/// - authenticated
/// - project:read
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/market",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<ProjectMarket>,
            description = "List all materials for a project",
            status = OK,
        ),
        (
            description = "There aren't any materials required for the project",
            status = NO_CONTENT,
        ),
        NotFound,
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    _identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let data =
        list_market(&state.postgres, &eve_gateway_api_client()?, project_id)
            .await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list_market(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClientItem,
    project_id: ProjectUuid,
) -> Result<Vec<ProjectMarket>> {
    let entries = sqlx::query!(
        r#"
            SELECT
                id,
                cost,
                type_id,
                quantity,
                source
            FROM project_market
            WHERE project_id = $1
        "#,
        *project_id,
    )
    .fetch_all(pool)
    .await
    .map_err(ProjectError::ListMisc)?;

    let type_ids = entries
        .iter()
        .map(|x| x.type_id)
        .map(Into::into)
        .collect::<Vec<_>>();

    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let mut project_market = Vec::new();
    for entry in entries.iter() {
        let item = if let Some(x) = items.get(
            &entry
                .type_id
                .into(),
        ) {
            x
        } else {
            continue;
        };

        let project_group = ProjectMarket {
            id:       entry.id,
            item:     item.clone(),
            quantity: entry.quantity,

            cost:   entry.cost,
            source: entry
                .source
                .clone(),
        };
        project_market.push(project_group);
    }

    Ok(sort_market(project_market))
}

sort_by_market_group_flat!(sort_market, ProjectMarket);
