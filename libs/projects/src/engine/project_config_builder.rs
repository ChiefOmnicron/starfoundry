use starfoundry_libs_structures::Structure;
use starfoundry_libs_types::{SystemId, TypeId};
use std::collections::HashMap;

use crate::{BlueprintBonus, StructureMapping};
use super::Dependency;
use super::project_config::ProjectConfig;

#[derive(Debug)]
pub struct ProjectConfigBuilder {
    bluprint_overwrite: HashMap<TypeId, BlueprintBonus>,
    dependencies:       Vec<Dependency>,
    /// list of items that should not be build
    blacklist:          Vec<TypeId>,
    /// list of structures
    structures:         Vec<Structure>,
    /// mapping from structure to categories
    structure_mappings: Vec<StructureMapping>,

    /// maximum amount of runs that can be done on a blueprint/formula
    max_runs:            HashMap<TypeId, u32>,
    /// max time a job can take before it is split
    max_time:           i32,

    /// does not calculate children
    skip_children:      bool,

    system_index:       HashMap<SystemId, (f32, f32)>,
    material_cost:      HashMap<TypeId, f64>,
}

impl ProjectConfigBuilder {
    pub fn add_blueprint_overwrites(
        mut self,
        overwrites: HashMap<TypeId, BlueprintBonus>,
    ) -> Self {
        overwrites
            .into_iter()
            .for_each(|(type_id, blueprint_bonus)| {
                self.bluprint_overwrite.insert(type_id, blueprint_bonus);
            });
        self
    }

    pub fn add_blacklists<T: Into<TypeId>>(
        mut self,
        type_ids: Vec<T>,
    ) -> Self {
        let entries = type_ids
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        self.blacklist.extend(entries);
        self
    }

    pub fn add_structures(
        mut self,
        structures: Vec<Structure>,
    ) -> Self {
        self.structures.extend(structures);
        self
    }

    pub fn add_structure_mappings(
        mut self,
        structure_mappings: Vec<StructureMapping>,
    ) -> Self {
        self.structure_mappings.extend(structure_mappings);
        self
    }

    pub fn add_max_run(
        mut self,
        type_id: TypeId,
        runs:    u32,
    ) -> Self {
        self.max_runs.insert(type_id, runs);
        self
    }

    pub fn set_max_runs(
        mut self,
        max_runs: HashMap<TypeId, u32>,
    ) -> Self {
        self.max_runs = max_runs;
        self
    }

    pub fn set_system_index(
        mut self,
        system_index: HashMap<SystemId, (f32, f32)>,
    ) -> Self {
        self.system_index = system_index;
        self
    }

    pub fn set_material_cost(
        mut self,
        material_cost: HashMap<TypeId, f64>,
    ) -> Self {
        self.material_cost = material_cost;
        self
    }

    pub fn set_skip_chidren(
        mut self,
        skip_children: bool
    ) -> Self {
        self.skip_children = skip_children;
        self
    }

    pub fn build(self) -> ProjectConfig {
        ProjectConfig {
            blueprint_overwrite: self.bluprint_overwrite,
            dependencies:        self.dependencies,
            blacklist:           self.blacklist,
            structures:          self.structures,
            structure_mappings:  self.structure_mappings,

            max_runs:            self.max_runs,
            max_time:            self.max_time,

            skip_children:       self.skip_children,

            system_index:        self.system_index,
            material_cost:       self.material_cost,
        }
    }
}

impl Default for ProjectConfigBuilder {
    fn default() -> Self {
        Self {
            bluprint_overwrite: default_blueprint_overwrites(),
            dependencies:       Vec::new(),
            blacklist:          Vec::new(),
            structures:         Vec::new(),
            structure_mappings: Vec::new(),

            max_runs:           HashMap::new(),
            // Three days in seconds
            max_time:           259_200i32,

            skip_children:      false,

            system_index:       HashMap::new(),
            material_cost:      HashMap::new(),
        }
    }
}

/// There are a couple blueprints that can never be above ME 0, TE 0, these are
/// added here
/// The list is not complete
/// 
fn default_blueprint_overwrites() -> HashMap<TypeId, BlueprintBonus> {
    vec![
        // TODO: fetch them from the database
        // Zirnitra
        (52907.into(), BlueprintBonus::no_bonus(52907.into())),
        // Capital Ultratidal Entropic Unit
        (53035.into(), BlueprintBonus::no_bonus(53035.into())),
        // Capital Radiation Conversion Unit
        (53036.into(), BlueprintBonus::no_bonus(53036.into())),
        // Capital Absorption Thruster Array
        (53037.into(), BlueprintBonus::no_bonus(53037.into())),

        // Ultratidal Entropic Disintegrator I
        (73793.into(), BlueprintBonus::no_bonus(52915.into())),

        // Naglfar Fleet Issue
        (73787.into(), BlueprintBonus::no_bonus(73787.into())),
        // Revelation Navy Issue
        (73790.into(), BlueprintBonus::no_bonus(73790.into())),
        // Moros Navy Issue
        (73792.into(), BlueprintBonus::no_bonus(73792.into())),
        // Phoenix Navy Issue
        (73793.into(), BlueprintBonus::no_bonus(73793.into())),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>()
}
