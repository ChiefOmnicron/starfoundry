use sqlx::PgPool;
use starfoundry_lib_types::StructureId;

use crate::eve_client::EveApiClient;
use crate::structure::error::{Result, StructureError};
use crate::structure::resolve_structure::model::{EveStructure, ResolveStructureResponse};
use crate::structure::list_structure_rigs::list_structure_rigs;
use crate::structure::fetch_services::fetch_services;

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

    let item = crate::item::fetch::fetch_item(pool, response.type_id)
        .await?
        .ok_or(StructureError::ItemNotFound)?;
    let system = crate::universe::fetch::fetch(
            pool,
            response.system_id,
        )
        .await?
        .ok_or(StructureError::SystemNotFound)?;
    let rigs = list_structure_rigs(
            pool,
            item.type_id,
        )
        .await?;
    let services = fetch_services(
            pool,
            item.type_id,
        )
        .await?;

    Ok(
        ResolveStructureResponse {
            structure_id:   structure_id,
            name:           response.name,
            system:         system,
            item:           item,
            position:       response.position,
            rigs:           rigs,
            services:       services,
        }
    )
}
