use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;
use crate::structure::list::filter::StructureFilter;
use crate::structure::{Structure, StructureError};
use crate::structure::error::Result;
use futures::future::try_join_all;

pub async fn list(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    filter:                 StructureFilter,
) -> Result<Vec<Structure>> {
    let entries = sqlx::query!(r#"
            SELECT structure.id
            FROM structure
            WHERE
                NOT (LOWER(structure.name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (structure.type_id = $3) IS FALSE AND
                NOT ($4::INTEGER IS NULL OR $4::INTEGER = ANY(services)) IS FALSE AND
                owner = $1
            ORDER BY structure.name
        "#,
            *character_id,
            filter.name,
            filter.structure_type_id,
            filter.service_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StructureError::ListStructures(e))?;

    let mut structures = Vec::new();
    let mut requests = Vec::new();
    for entry in entries {
        let structure_uuid = entry.id.into();
        requests.push(Structure::new(pool, eve_gateway_api_client, structure_uuid));
    }

    for request in try_join_all(requests).await.unwrap_or_default() {
        let structure = if let Some(x) = request {
            x
        } else {
            continue
        };

        structures.push(structure);
    }

    Ok(structures)
}
