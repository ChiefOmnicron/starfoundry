use serde::{Deserialize, Serialize};
use starfoundry_libs_types::{CategoryId, GroupId, TypeId};
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Item {
    pub name:          String,
    pub volume:        f32,

    pub category_id:   CategoryId,
    pub group_id:      GroupId,
    pub type_id:       TypeId,
    pub meta_group_id: Option<GroupId>,
    pub repackaged:    Option<i32>,
}
