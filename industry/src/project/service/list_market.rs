use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientItem, Item};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;
use crate::sort_by_market_group_flat;

pub async fn list_market(
    pool:                   &PgPool,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClientItem,
) -> Result<Vec<ProjectMarket>> {
    let entries = sqlx::query!(r#"
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
        let item = if let Some(x) = items.get(&entry.type_id.into()) {
            x
        } else {
            continue;
        };

        let project_group = ProjectMarket {
            id:         entry.id.into(),
            item:       item.clone(),
            quantity:   entry.quantity,

            cost:       entry.cost,
            source:     entry.source.clone(),
        };
        project_market.push(project_group);
    }

    Ok(sort_market(project_market))
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMarket {
    pub id:       Uuid,
    pub item:     Item,
    pub quantity: i32,

    pub cost:     Option<f64>,
    pub source:   Option<String>,
}

sort_by_market_group_flat!(sort_market, ProjectMarket);
