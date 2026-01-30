use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::Item;
use starfoundry_lib_types::TypeId;

use crate::industry::error::{IndustryError, Result};
use crate::industry::calculation::models::BlueprintTyp;

/// Single dependency that represents either a end product, component or
/// material
/// 
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependency {
    pub blueprint_type_id:   TypeId,
    pub product_type_id:     TypeId,
    pub needed:              f32,
    pub time:                f32,
    pub produces:            i32,
    pub item:                Item,
    pub components:          Vec<Dependency>,
    pub typ:                 BlueprintTyp,
}

impl Dependency {
    pub fn try_from(
        quantity: u32,
        value:    serde_json::Value
    ) -> Result<Self> {
        let mut dependency: Dependency = serde_json::from_value(value)
            .map_err(IndustryError::ParseJsonToDependency)?;
        dependency.needed = quantity as f32;
        Ok(dependency)
    }
}
