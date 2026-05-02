use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_industry::ProjectUuid;
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::project::error::{ProjectError, Result};

pub async fn list_products(
    pool:                   &PgPool,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<Vec<ProjectProduct>> {
    let entries = sqlx::query!(r#"
            SELECT
                type_id,
                quantity,
                material_efficiency
            FROM solution_product sp
            JOIN project p ON p.solution_id = sp.solution_id
            WHERE p.id = $1
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

        let product = ProjectProduct {
            item:                   item,
            quantity:               entry.quantity,
            material_efficiency:    entry.material_efficiency,
        };
        result.push(product);
    }

    Ok(result)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectProduct {
    pub item:                   Item,
    pub quantity:               i32,
    pub material_efficiency:    i32,
}

