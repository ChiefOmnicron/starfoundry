use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::{ProjectUuid, Structure};
use starfoundry_lib_types::CharacterId;

use crate::project::error::{ProjectError, Result};
use crate::structure::service::FetchStructureQuery;

pub async fn list_market_structures(
    pool:                   &PgPool,
    character_id:           CharacterId,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_id:             ProjectUuid,
) -> Result<Vec<Structure>> {
    let markets = sqlx::query!("
            SELECT structure_id
            FROM project p
            JOIN project_group_default_market pgdm ON pgdm.project_group_id = p.project_group_id
            WHERE p.id = $1
        ",
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ProjectError::Fetch(e, project_id))?
        .into_iter()
        .map(|x| x.structure_id.into())
        .collect::<Vec<_>>();

    let mut structures = Vec::new();
    for market in markets {
        if let Some(x) = crate::structure::service::fetch(
            pool,
            eve_gateway_api_client,
            character_id,
            market,
            FetchStructureQuery::default(),
        ).await? {
            structures.push(x);
        } else {
            // silently ignore errors
            tracing::debug!("Couldn't find structure {}", market);
            continue;
        }
    }

    Ok(structures)
}
