use serde::Serialize;
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[schema(
    example = json!({
        "base_price": null,
        "category_id": 6,
        "group_id": 30,
        "meta_group_id": null,
        "name": "Ragnarok",
        "repackaged": 10000000,
        "type_id": 23773,
        "volume": 100000000
    })
)]
pub struct Item {
    pub type_id:        TypeId,
    pub category_id:    CategoryId,
    pub group_id:       GroupId,
    pub volume:         f32,
    pub name:           String,

    pub meta_group_id:  Option<GroupId>,
    pub repackaged:    Option<i32>,
}
