use crate::{Item, System};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use starfoundry_lib_types::{StructureId, TypeId};

/// Return message for resolving a structure
#[derive(Debug, Deserialize, Serialize, ToSchema)]
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
    pub position:       StructurePosition,
    /// All rigs that can be installed into the structure
    pub rigs:           Vec<StructureRigResponse>,
    /// All services that can be installed into the structure
    pub services:       StructureServiceResponse,
}

/// Coordinates of a structure within the system
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct StructurePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct StructureServiceResponse {
    pub services: Vec<Item>,
    pub slots:    i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct StructureRigResponse {
    pub item:            Item,
    pub excludes:        Vec<TypeId>,

    pub material:        Option<f32>,
    pub time:            Option<f32>,
    pub category_groups: Vec<i32>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum BonusModifier {
    ManufacturingMaterial,
    ManufactureTime,
    ReactionMaterial,
    ReactionTime,
}
