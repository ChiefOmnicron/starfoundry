use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn list_default_blueprint_overwrite(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<Vec<BlueprintOverwrite>> {
    let entries = sqlx::query!("
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
        .map_err(|e| ProjectGroupError::FetchGroupDefaults(e, project_group_uuid))?;

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

    let mut blueprint_overwrites = Vec::new();
    for entry in entries {
        if let Some(x) = items.get(&entry.type_id.into()) {
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
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::eve_gateway_api_client;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::list_default_blueprint_overwrite(
                &pool,
                &eve_gateway_api_client().unwrap(),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 1);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let response = super::list_default_blueprint_overwrite(
                &pool,
                &eve_gateway_api_client().unwrap(),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 0);
    }
}
