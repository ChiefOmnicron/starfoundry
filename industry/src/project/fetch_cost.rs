use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project::error::{ProjectError, Result};
use crate::project::fetch::ProjectCost;
use crate::{AppState, ProjectUuid};

/// Fetch Cost
///
/// - Alternative route: `/latest/structures/{ProjectUuid}/cost`
/// - Alternative route: `/v1/structures/{ProjectUuid}/cost`
///
/// ---
///
/// Gets the cost of the project
///
/// ## Security
/// - authenticated
/// - structure:read
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/cost",
    tag = "Projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = ProjectCost,
            description = "Project costs",
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
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let entry =
        fetch_cost(&state.postgres, identity.character_id, project_id).await?;

    if let Some(x) = entry {
        Ok((StatusCode::OK, Json(x)).into_response())
    } else {
        Ok((StatusCode::NO_CONTENT, Json(())).into_response())
    }
}

pub async fn fetch_cost(
    pool: &PgPool,
    character_id: CharacterId,
    project_id: ProjectUuid,
) -> Result<Option<ProjectCost>> {
    let sell_price = sqlx::query!(
        "
            SELECT sell_price
            FROM project
            WHERE
                (owner = $1 OR owner = 0) AND
                id = $2
        ",
        *character_id,
        *project_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ProjectError::FetchCost(e, project_id))?;

    let job_cost = sqlx::query!(
        "
            SELECT SUM(cost) AS cost
            FROM project_job
            WHERE project_id = $1
        ",
        *project_id,
    )
    .fetch_optional(pool)
    .await
    .map(|x| {
        x.map(|x| x.cost)
            .unwrap_or_default()
    })
    .map_err(|e| ProjectError::FetchCost(e, project_id))?;
    let market_cost = sqlx::query!(
        "
            SELECT SUM(cost) AS cost
            FROM project_market
            WHERE project_id = $1
        ",
        *project_id,
    )
    .fetch_optional(pool)
    .await
    .map(|x| {
        x.map(|x| x.cost)
            .unwrap_or_default()
    })
    .map_err(|e| ProjectError::FetchCost(e, project_id))?;
    let misc_cost = sqlx::query!(
        "
            SELECT SUM(cost) AS cost
            FROM project_misc
            WHERE project_id = $1
        ",
        *project_id,
    )
    .fetch_optional(pool)
    .await
    .map(|x| {
        x.map(|x| x.cost)
            .unwrap_or_default()
    })
    .map_err(|e| ProjectError::FetchCost(e, project_id))?;
    let excess_cost = sqlx::query!(
        "
            SELECT SUM(cost) AS cost
            FROM project_excess
            WHERE project_id = $1
        ",
        *project_id,
    )
    .fetch_optional(pool)
    .await
    .map(|x| {
        x.map(|x| x.cost)
            .unwrap_or_default()
    })
    .map_err(|e| ProjectError::FetchCost(e, project_id))?;
    let stock_cost = sqlx::query!(
        "
            SELECT SUM(cost) AS cost
            FROM project_stock
            WHERE project_id = $1
        ",
        *project_id,
    )
    .fetch_optional(pool)
    .await
    .map(|x| {
        x.map(|x| x.cost)
            .unwrap_or_default()
    })
    .map_err(|e| ProjectError::FetchCost(e, project_id))?;

    if let Some(x) = sell_price {
        Ok(Some(ProjectCost {
            sell_price: x
                .sell_price
                .unwrap_or_default(),

            job_cost:    job_cost.unwrap_or_default(),
            market_cost: market_cost.unwrap_or_default(),
            misc_cost:   misc_cost.unwrap_or_default(),
            excess_cost: excess_cost.unwrap_or_default(),
            stock_cost:  stock_cost.unwrap_or_default(),
        }))
    } else {
        Ok(None)
    }
}
