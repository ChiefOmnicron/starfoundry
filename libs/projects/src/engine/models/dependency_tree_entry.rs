use serde::Serialize;
use starfoundry_lib_items::Item;
use starfoundry_lib_structures::Structure;
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

use super::{Bonus, DependencyBuildCost};
use crate::BlueprintTyp;

#[derive(Clone, Debug, Serialize)]
pub struct DependencyTreeEntry {
    pub blueprint_type_id:  TypeId,
    pub product_type_id:    TypeId,
    pub needed:             f32,
    pub time:               f32,
    pub produces:           i32,
    // Number of runs splitted by the maximum time they are allowed to take
    pub runs:               Vec<u32>,
    pub children:           HashMap<TypeId, f32>,
    #[serde(skip)]
    pub children_unbonused: HashMap<TypeId, f32>,
    pub typ:                BlueprintTyp,
    pub item:               Item,
    pub stock:              i32,
    pub is_product:         bool,
    /// Estimate of the build cost
    pub build_cost:         DependencyBuildCost,
    pub structure:          Option<Structure>,
    pub bonus:              Vec<Bonus>,
}
