use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_industry::ProjectUuid;
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::project::error::{ProjectError, Result};
use crate::sort_by_market_group_flat;

pub async fn list_excess(
    pool:                   &PgPool,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<Vec<ProjectExcess>> {
    let entries = sqlx::query!(r#"
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
        .map(|x| x.type_id.into())
        .collect::<Vec<_>>();
    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let mut result = Vec::new();
    for entry in entries {
        let item = if let Some(x) = items.get(&entry.type_id.into()) {
            x.clone()
        } else {
            continue;
        };

        let product = ProjectExcess {
            item:       item,
            quantity:   entry.quantity,
            cost:       entry.cost,
        };
        result.push(product);
    }

    Ok(sort_market(result))
}

sort_by_market_group_flat!(sort_market, ProjectExcess);

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectExcess {
    pub item:       Item,
    pub quantity:   i32,
    pub cost:       Option<f64>,
}

