use starfoundry_lib_items::Item;
use starfoundry_lib_types::TypeId;
use serde::Deserialize;

use crate::BlueprintTyp;

/// Single dependency that represents either a end product, component or
/// material
/// 
#[derive(Clone, Debug, Deserialize)]
pub struct Dependency {
    pub btype_id:   TypeId,
    pub ptype_id:   TypeId,
    pub needed:     f32,
    pub time:       f32,
    pub produces:   i32,
    pub item:       Item,
    pub components: Vec<Dependency>,
    pub typ:        BlueprintTyp,
}

impl Dependency {
    pub fn try_from(
        quantity: u32,
        value:    serde_json::Value
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut dependency: Dependency = serde_json::from_value(value)?;
        dependency.needed = quantity as f32;
        Ok(dependency)
    }
}
