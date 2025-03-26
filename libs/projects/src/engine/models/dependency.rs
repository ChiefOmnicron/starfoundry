use serde::{Deserialize, Serialize};
use starfoundry_libs_items::Item;
use starfoundry_libs_types::TypeId;

use crate::{BlueprintTyp, Error, Result};

/// Single dependency that represents either a end product, component or
/// material
/// 
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    ) -> Result<Self> {
        let mut dependency: Dependency = serde_json::from_value(value)
            .map_err(Error::ParseJsonToDependency)?;
        dependency.needed = quantity as f32;
        Ok(dependency)
    }
}
