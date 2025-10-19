use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{StructureId, SystemId, TypeId};
use utoipa::ToSchema;

use crate::item::Item;
use crate::universe::fetch_system::System;
use crate::structure::list_structure_rigs::StructureRigResponse;
use crate::structure::fetch_services::StructureServiceResponse;

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
    /// [TypeId] of the structure
    pub position:  EvePosition,
}

/// Coordinates of a structure within the system
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct EvePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Return message for resolving a structure
#[derive(Debug, Serialize, ToSchema)]
pub struct ResolveStructureResponse {
    /// ID of the in-game structure
    pub structure_id:   StructureId,
    /// Name of the structure
    pub name:           String,
    /// Id of the system the structure is located in
    pub system:         System,
    /// [TypeId] of the structure
    pub item:           Item,
    /// Location in the universe
    pub position:       EvePosition,
    /// All rigs that can be installed into the structure
    pub rigs:           Vec<StructureRigResponse>,
    /// All services that can be installed into the structure
    pub services:       StructureServiceResponse,
}
