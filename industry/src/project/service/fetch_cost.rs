use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use starfoundry_lib_industry::ProjectUuid;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::project::error::ProjectError;
use crate::project::error::Result;

pub async fn fetch_cost(
    pool:           &PgPool,
    character_id:   CharacterId,
    project_id:     ProjectUuid,
) -> Result<Option<ProjectCost>> {
    let sell_price = sqlx::query!("
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

    let job_cost = sqlx::query!("
            SELECT SUM(cost) AS cost
            FROM project_job
            WHERE project_id = $1
        ",
            *project_id,
        )
        .fetch_optional(pool)
        .await
        .map(|x| x.map(|x| x.cost).unwrap_or_default())
        .map_err(|e| ProjectError::FetchCost(e, project_id))?;
    let market_cost = sqlx::query!("
            SELECT SUM(cost) AS cost
            FROM project_market
            WHERE project_id = $1
        ",
            *project_id,
        )
        .fetch_optional(pool)
        .await
        .map(|x| x.map(|x| x.cost).unwrap_or_default())
        .map_err(|e| ProjectError::FetchCost(e, project_id))?;
    let misc_cost = sqlx::query!("
            SELECT SUM(cost) AS cost
            FROM project_misc
            WHERE project_id = $1
        ",
            *project_id,
        )
        .fetch_optional(pool)
        .await
        .map(|x| x.map(|x| x.cost).unwrap_or_default())
        .map_err(|e| ProjectError::FetchCost(e, project_id))?;
    let excess_cost = sqlx::query!("
            SELECT SUM(cost) AS cost
            FROM project_excess
            WHERE project_id = $1
        ",
            *project_id,
        )
        .fetch_optional(pool)
        .await
        .map(|x| x.map(|x| x.cost).unwrap_or_default())
        .map_err(|e| ProjectError::FetchCost(e, project_id))?;
    let stock_cost = sqlx::query!("
            SELECT SUM(cost) AS cost
            FROM project_stock
            WHERE project_id = $1
        ",
            *project_id,
        )
        .fetch_optional(pool)
        .await
        .map(|x| x.map(|x| x.cost).unwrap_or_default())
        .map_err(|e| ProjectError::FetchCost(e, project_id))?;

    if let Some(x) = sell_price {
        Ok(Some(ProjectCost {
            sell_price:     x.sell_price.unwrap_or_default(),

            job_cost:       job_cost.unwrap_or_default(),
            market_cost:    market_cost.unwrap_or_default(),
            misc_cost:      misc_cost.unwrap_or_default(),
            excess_cost:    excess_cost.unwrap_or_default(),
            stock_cost:     stock_cost.unwrap_or_default(),
        }))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectCost {
    sell_price:     f64,

    job_cost:       f64,
    market_cost:    f64,
    misc_cost:      f64,
    excess_cost:    f64,
    stock_cost:     f64,
}
