use serde::Serialize;
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct StructureRig {
    /// name of the rig, wihtout `Standup ?-Size`
    pub name:            String,
    /// [TypeId] of the rig
    pub type_id:         TypeId,

    pub material:        Option<f32>,
    pub time:            Option<f32>,
    pub category_groups: Vec<i32>,
}

impl StructureRig {
    /// Checks if the rig has bonuses to the given category or group
    pub fn has_category_or_group(
        &self,
        category_or_group: i32,
    ) -> bool {
        self.category_groups.contains(&category_or_group)
    }
}
