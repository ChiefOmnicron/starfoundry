use serde::Serialize;
use starfoundry_libs_structures::Structure;
use starfoundry_libs_types::{SystemId, TypeId};
use std::collections::HashMap;

use super::{Dependency, ProjectConfigBuilder};
use crate::{BlueprintBonus, StructureMapping};

/// Configration for a new project.
/// Must be created with [ProjectConfigBuilder].
/// 
/// TODO: rename
#[derive(Debug, Serialize)]
pub struct ProjectConfig {
    pub(crate) blueprint_overwrite: HashMap<TypeId, BlueprintBonus>,
    pub(crate) dependencies:        Vec<Dependency>,

    /// list of items that should not be build
    pub(crate) blacklist:           Vec<TypeId>,
    /// list of structures
    pub(crate) structures:          Vec<Structure>,
    /// mapping from structure to categories
    pub(crate) structure_mappings:  Vec<StructureMapping>,

    /// maximum amount of runs that can be done on a blueprint/formula
    pub(crate) max_runs:            HashMap<TypeId, u32>,
    /// max time a job can take before it is split
    pub(crate) max_time:            i32,

    /// does not calculate children
    pub(crate) skip_children:       bool,

    pub(crate) system_index:        HashMap<SystemId, (f32, f32)>,
    pub(crate) material_cost:       HashMap<TypeId, f64>,
}

impl ProjectConfig {
    pub fn is_blacklisted<T: Into<TypeId>>(
        &self,
        type_id: T
    ) -> bool {
        self.blacklist.contains(&type_id.into())
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        ProjectConfigBuilder::default().build()
    }
}
