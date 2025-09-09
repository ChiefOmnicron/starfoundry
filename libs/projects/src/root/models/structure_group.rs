use starfoundry_lib_structures::Structure;
use starfoundry_lib_types::SystemId;
use uuid::Uuid;

use super::StructureMapping;

#[derive(Clone, Debug, Default)]
pub struct ProjectStructureGroup {
    pub id:         Uuid,
    pub structures: Vec<Structure>,
    pub mapping:    Vec<StructureMapping>,
    pub system_ids: Vec<SystemId>,
}
