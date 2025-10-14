use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;
use crate::structure::list::filter::StructureFilter;
use crate::structure::{Structure, StructureError};
use crate::structure::error::Result;

pub async fn list(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    filter:                 StructureFilter,
) -> Result<Vec<Structure>> {
    let entries = sqlx::query!(r#"
            SELECT structure.id
            FROM structure
            JOIN system ON system.system_id = structure.system_id
            WHERE
                NOT (LOWER(structure.name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (structure.system_id = $3) IS FALSE AND
                NOT (structure.type_id = $4) IS FALSE AND
                NOT ($5::INTEGER IS NULL OR $5::INTEGER = ANY(services)) IS FALSE AND
                (owner = $1 OR owner = 0)
                ORDER BY structure.name
        "#,
            *character_id,
            filter.name,
            filter.system_id,
            filter.structure_type_id,
            filter.service_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StructureError::ListStructures(e))?;

    let mut structures = Vec::new();
    for entry in entries {
        let structure_uuid = entry.id.into();
        let structure = if let Ok(Some(x)) = Structure::new(pool, eve_gateway_api_client, structure_uuid).await {
            x
        } else {
            continue
        };

        structures.push(structure);
    }

    Ok(structures)
}
