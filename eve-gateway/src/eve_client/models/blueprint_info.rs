use serde::Deserialize;
use starfoundry_lib_types::{ItemId, LocationId, TypeId};

/// Information of a blueprint
#[derive(Clone, Debug, Deserialize)]
pub struct BlueprintInfo {
    /// Unique EVE Item ID
    pub item_id:             ItemId,
    /// Location of the blueprint
    pub location_flag:       String,
    /// Location ID of the structure the blueprint is in
    pub location_id:         LocationId,
    /// Material efficiency of the blueprint
    pub material_efficiency: u32,
    /// range of numbers with a minimum of -2 and no maximum value where -1 is
    /// an original and -2 is a copy. It can be a positive integer if it is a
    /// stack of blueprint originals fresh from the market (e.g. no activities performed on them yet).
    pub quantity:            i32,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original.
    pub runs:                i32,
    /// Time efficiency of the blueprint
    pub time_efficiency:     u32,
    /// TypeID of the blueprint
    pub type_id:             TypeId,
}
