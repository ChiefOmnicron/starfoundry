use serde::Deserialize;
use starfoundry_lib_types::{ItemId, LocationId, TypeId};

/// Represents an asset
#[derive(Debug, Deserialize)]
pub struct AssetEntry {
    /// Unique Id of the item
    pub item_id:           ItemId,
    /// Flag of the location, eg. MedSlot6, Deliveries, Wallet
    pub location_flag:     String,
    /// Either a id of a structure, container or ship
    pub location_id:       LocationId,
    /// Stored quantity
    pub quantity:          i32,
    /// [TypeId] of the item
    pub type_id:           TypeId,
    /// Location type
    pub location_type:     String,

    /// True if the item is a copy
    #[serde(default)]
    pub is_blueprint_copy: bool,
}
