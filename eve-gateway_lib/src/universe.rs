use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{ConstellationId, RegionId, StructureId, SystemId};
use utoipa::ToSchema;

use crate::Item;

/// Return message for resolving a structure
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ResolveStructureResponse {
    /// ID of the in-game structure
    pub structure_id:   StructureId,
    /// Name of the structure
    pub name:           String,
    /// Id of the system the structure is located in
    pub system_id:      System,
    /// [TypeId] of the structure
    pub type_id:        Item,
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

// TODO: move me when a better location is found
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "region_id": 10000001,
        "constellation_id": 20000001,
        "system_id": 30000001,
        "region_name": "Derelik",
        "constellation_name": "San Matar",
        "system_name": "Tanoo",
        "security": 0.858324
    })
)]
pub struct System {
    pub region_id:          RegionId,
    pub region_name:        String,
    pub constellation_id:   ConstellationId,
    pub constellation_name: String,
    pub system_id:          SystemId,
    pub system_name:        String,
    pub security:           f32,
}
