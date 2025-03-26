use starfoundry_libs_structures::Structure;
use starfoundry_libs_types::SystemId;

use super::StructureMapping;

#[derive(Debug, Default)]
pub struct ProjectStructureGroup {
    pub structures: Vec<Structure>,
    pub mapping:    Vec<StructureMapping>,
    pub system_ids: Vec<SystemId>,
}
