use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;

use crate::api_docs::{
    BadRequest, InternalServerError, NotFound, Unauthorized
};
use crate::project::error::{ProjectError, Result};
use crate::project::list::ProjectExcess;
use crate::{
    AppState, ProjectUuid, eve_gateway_api_client, sort_by_market_group_flat
};

/// List Excess
///
/// - Alternative route: `/latest/projects/{ProjectUuid}/excess`
/// - Alternative route: `/v1/projects/{ProjectUuid}/excess`
///
/// ---
///
/// Lists all excess entries that belong to the project
///
/// ## Security
/// - authenticated
/// - project:read
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/excess",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<ProjectExcess>,
            description = "List all misc entries for a project",
            status = OK,
        ),
        (
            description = "There aren't any misc entries",
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
    State(state): State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let data =
        list_excess(&state.postgres, &eve_gateway_api_client()?, project_id)
            .await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list_excess(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_id: ProjectUuid,
) -> Result<Vec<ProjectExcess>> {
    let entries = sqlx::query!(
        r#"
            SELECT
                type_id,
                quantity,
                cost
            FROM project_excess
            WHERE project_id = $1
        "#,
        *project_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ProjectError::Fetch(e, project_id))?;

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

    let mut result = Vec::new();
    for entry in entries {
        let item = if let Some(x) = items.get(
            &entry
                .type_id
                .into(),
        ) {
            x.clone()
        } else {
            continue;
        };

        let product = ProjectExcess {
            item:     item,
            quantity: entry.quantity,
            cost:     entry.cost,
        };
        result.push(product);
    }

    Ok(sort_market(result))
}

sort_by_market_group_flat!(sort_market, ProjectExcess);
