use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{ResolveStructureResponse, StructurePosition};
use starfoundry_lib_types::{StructureId, SystemId, TypeId};

use crate::eve_client::EveApiClient;
use crate::structure::error::{Result, StructureError};
use crate::structure::services::{fetch_services, list_structure_rigs};

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
        .fetch_auth(&path, &[])
        .await?;

    let item = crate::item::services::fetch_item(pool, response.type_id)
        .await?
        .ok_or(StructureError::ItemNotFound)?;
    let system = crate::universe::services::fetch_system(
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
            structure_id:         structure_id,
            name:                 response.name,
            system:               system,
            item:                 item,
            position:             response.position,
            installable_rigs:     rigs,
            installable_services: services,
        }
    )
}

/// Represents a structure
#[derive(Debug, Deserialize, Serialize)]
pub struct EveStructure {
    /// Name of the structure
    pub name:      String,
    /// Id of the system the structure is located in
    #[serde(alias = "solar_system_id")]
    pub system_id: SystemId,
    /// [TypeId] of the structure
    pub type_id:   TypeId,
    /// Position of the structure in space
    pub position:  StructurePosition,
}
