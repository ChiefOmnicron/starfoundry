use serde::Serialize;
use starfoundry_lib_types::{ConstellationId, RegionId, SystemId};
use utoipa::ToSchema;

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

/// Determines in what security status the system is located in
/// 
#[derive(Clone, Debug, Serialize, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "SYSTEM_SECURITY")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum Security {
    Highsec,
    Lowsec,
    Nullsec,
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
