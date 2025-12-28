use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "base_price": null,
        "category": {
            "category_id": 0,
            "name": "#System"
        },
        "group": {
            "group_id": 0,
            "category_id": 0,
            "name": "#System"
        },
        "meta_group_id": null,
        "name": "Ragnarok",
        "repackaged": 10000000,
        "type_id": 23773,
        "volume": 100000000
    })
)]
pub struct Item {
    pub type_id:    TypeId,
    pub category:   Category,
    pub group:      Group,
    pub volume:     f32,
    pub name:       String,

    pub meta_group: Option<GroupId>,
    pub repackaged: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "category_id": 0,
        "name": "#System"
    })
)]
pub struct Category {
    pub category_id: CategoryId,
    pub name:        String,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "group_id": 0,
        "category_id": 0,
        "name": "#System"
    })
)]
pub struct Group {
    pub group_id:    GroupId,
    pub category_id: CategoryId,
    pub name:        String,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, IntoParams)]
#[schema(
    example = json!({
        "name": "Pyerite"
    })
)]
pub struct ListItemFilter {
    pub name: String,

    /// Only searches for items that are buildable
    #[serde(default)]
    pub buildable: Option<bool>,
    /// Only searches for blueprints
    #[serde(default)]
    pub blueprint: Option<bool>,
}
