use serde::{Deserialize, Serialize};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::item::Item;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct StructureRigResponse {
    pub item:            Item,
    pub excludes:        Vec<TypeId>,

    pub material:        Option<f32>,
    pub time:            Option<f32>,
    pub category_groups: Vec<i32>,
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
