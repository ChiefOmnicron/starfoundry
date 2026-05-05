use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::ProjectUuid;
use std::collections::HashMap;

use crate::project::error::{ProjectError, Result};
use crate::sort_by_market_group_flat;
use starfoundry_lib_industry::project::ProjectExcess;

pub async fn list_excess(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_id:             ProjectUuid,
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
