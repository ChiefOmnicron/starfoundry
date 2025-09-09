mod calculation;
mod models;
mod project_config_builder;
mod project_config;
mod result;

use serde::Deserialize;
use serde::Serialize;
use starfoundry_lib_structures::Structure;
use starfoundry_lib_structures::StructureUuid;
use starfoundry_lib_types::SystemId;
use starfoundry_lib_types::TypeId;
use uuid::Uuid;

pub use self::models::*;

pub use self::calculation::*;
pub use self::project_config::*;
pub use self::project_config_builder::*;
pub use self::result::*;

#[derive(Copy, Clone, Debug)]
pub struct BlueprintBonus {
    pub ptype_id: TypeId,
    pub material: f32,
    pub time:     f32,
}

impl BlueprintBonus {
    pub fn no_bonus(ptype_id: TypeId) -> Self {
        Self {
            ptype_id,
            material: 0f32,
            time:     0f32,
        }
    }
}

#[derive
(
    Copy, Clone, Debug,
    Eq, PartialEq,
    Deserialize, Serialize,
)]
pub enum BlueprintTyp {
    Blueprint,
    Reaction,
    Material,
}

#[derive(Clone, Debug)]
pub struct StockMinimal {
    pub quantity:    i32,
    pub type_id:     TypeId,
}

#[derive(Clone, Debug, Default)]
pub struct ProjectStructureGroup {
    pub id:         Uuid,
    pub structures: Vec<Structure>,
    pub mapping:    Vec<StructureMapping>,
    pub system_ids: Vec<SystemId>,
}

#[derive(Clone, Debug, Serialize)]
pub struct StructureMapping {
    pub structure_uuid: StructureUuid,
    pub category_group: Vec<i32>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ViableMarketPrice {
    pub source:          String,
    pub type_id:         i32,
    pub quantity:        u64,
    pub remaining:       u64,
    pub price:           f64,
    /// if set to true, then there is no market to fulfill the request
    pub incomplete_data: bool,
}
