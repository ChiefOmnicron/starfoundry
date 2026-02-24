use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::Item;
use starfoundry_lib_types::{ItemId, LocationId};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Blueprint {
    pub item:                Item,
    pub material_efficiency: i32,
    pub time_efficiency:     i32,
    pub quantity:            i32,
    pub runs:                i32,

    pub item_id:             ItemId,
    pub location_id:         LocationId,
    pub location_flag:       String,
}
