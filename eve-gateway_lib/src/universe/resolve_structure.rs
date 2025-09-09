use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{StructureId, SystemId, TypeId};
use utoipa::ToSchema;

use crate::{EveGatewayClient, Result};

pub async fn resolve_structure(
    gateway_client: &impl EveGatewayClient,
    structure_id:   StructureId,
) -> Result<ResolveStructureResponse> {
    gateway_client
        .fetch(&format!("universe/structures/{}", *structure_id))
        .await
        .map_err(Into::into)
}

/// Return message for resolving a structure
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ResolveStructureResponse {
    /// ID of the in-game structure
    pub structure_id:   StructureId,
    /// Name of the structure
    pub name:           String,
    /// Id of the system the structure is located in
    /// TODO: add the usual information
    pub system_id:      SystemId,
    /// [TypeId] of the structure
    /// TODO: add the usual information
    pub type_id:        TypeId,
    /// [TypeId] of the structure
    pub position:       StructurePosition,
}

/// Coordinates of a structure within the system
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct StructurePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
