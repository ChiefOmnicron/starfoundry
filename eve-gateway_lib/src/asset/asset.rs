use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use starfoundry_lib_types::{ItemId, LocationId, TypeId};

use crate::asset::location_flag::LocationFlag;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Asset {
    /// Unique ID for this item
    pub item_id:                ItemId,
    /// References a station, a ship or an item_id if this blueprint is located within a container
    pub location_id:            LocationId,
    pub type_id:                TypeId,
    /// Type of the location_id
    pub location_flag:          LocationFlag,
    /// A range of numbers with a minimum of -2 and no maximum value where -1
    /// is an original and -2 is a copy. It can be a positive integer if it is
    /// a stack of blueprint originals fresh from the market (e.g. no activities
    /// performed on them yet).
    pub quantity:               i32,
    pub location_type:          String,

    pub is_blueprint_copy:      Option<bool>,
    pub is_singleton:           bool,
}
