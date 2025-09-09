use sqlx::PgPool;
use starfoundry_lib_types::StructureId;

use crate::eve_client::EveApiClient;
use crate::universe::error::{Result, UniverseError};
use crate::universe::resolve_structure::model::{EveStructure, ResolveStructureResponse};

/// Fetches information about the given location id.
/// The structure id must be larger than 1_000_000_000_000.
/// 
/// # Errors
/// 
/// - If the EVE API is not available
/// - If the [EveAuthClient] is not valid
/// - If the character does not have access to the structure
/// - If the structure does not exist
/// - If the [StructureId] is not a valid id
/// 
/// # Returns
/// Information about the structure
/// 
pub async fn resolve_structure(
    pool:           &PgPool,
    eve_api_client: EveApiClient,
    structure_id:   StructureId,
) -> Result<ResolveStructureResponse> {
    let path = format!(
        "latest/universe/structures/{}",
        structure_id
    );

    let response: EveStructure = eve_api_client
        .fetch_auth(&path)
        .await?;

    let item = crate::item::fetch(
            pool,
            response.type_id,
        )
        .await?
        .ok_or(UniverseError::ItemNotFound)?;
    let system = crate::universe::fetch_system::fetch(
            pool,
            response.system_id,
        )
        .await?
        .ok_or(UniverseError::ItemNotFound)?;

    Ok(
        ResolveStructureResponse {
            structure_id:   structure_id,
            name:           response.name,
            system_id:      system,
            type_id:        item,
            position:       response.position
        }
    )
}
