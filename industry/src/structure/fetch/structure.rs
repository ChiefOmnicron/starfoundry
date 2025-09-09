use serde::Serialize;
use starfoundry_lib_types::{ConstellationId, RegionId, SystemId};
use utoipa::ToSchema;

use crate::structure::models::Security;

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct StructureSystem {
    pub region_id:          RegionId,
    pub region_name:        String,

    pub constellation_id:   ConstellationId,
    pub constellation_name: String,

    pub system_id:          SystemId,
    pub system_name:        String,

    pub security:           f32,
    pub security_group:     Security,
}

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(type_name = "BONUS_MODIFIER")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum BonusModifier {
    ManufacturingMaterial,
    ManufactureTime,
    ReactionMaterial,
    ReactionTime,
}
