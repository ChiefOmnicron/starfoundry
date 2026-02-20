use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use starfoundry_lib_types::{CategoryId, GroupId, StructureId, TypeId};

use crate::{Category, Group, Item, System};

/// Return message for resolving a structure
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ResolveStructureResponse {
    /// ID of the in-game structure
    pub structure_id:         StructureId,
    /// Name of the structure
    pub name:                 String,
    /// Id of the system the structure is located in
    pub system:               System,
    /// [TypeId] of the structure
    pub item:                 Item,
    /// Location in the universe
    pub position:             StructurePosition,
    /// All rigs that can be installed into the structure
    pub installable_rigs:     Vec<StructureRigResponse>,
    /// All services that can be installed into the structure
    pub installable_services: StructureServiceResponse,
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
    pub item:       Item,
    pub excludes:   Vec<TypeId>,

    pub material:   Option<f32>,
    pub time:       Option<f32>,
    pub categories: Vec<Category>,
    pub groups:     Vec<Group>,
}

impl StructureRigResponse {
    pub fn has_category(
        &self,
        category_id: CategoryId,
    ) -> bool {
        self
            .categories
            .iter()
            .find(|x| x.category_id == category_id)
            .is_some()
    }

    pub fn has_group(
        &self,
        group_id: GroupId,
    ) -> bool {
        self
            .groups
            .iter()
            .find(|x| x.group_id == group_id)
            .is_some()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum BonusModifier {
    ManufactureMaterial,
    ManufactureTime,
    ReactionMaterial,
    ReactionTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructureRigBlueprintBonus {
    pub bonus_me:         f64,
    pub bonus_te:         f64,

    pub is_manufacturing: bool,
    pub is_reaction:      bool,

    pub blueprint:        Item,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlueprintBonusByRig {
    pub rigs:     Vec<TypeId>,
    pub services: Vec<TypeId>,
}
