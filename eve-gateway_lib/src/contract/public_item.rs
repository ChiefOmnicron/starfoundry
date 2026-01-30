use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{ItemId, TypeId};
use utoipa::ToSchema;


#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PublicContractItem {
    /// Number of items in the stack
    pub quantity:               i32,
    /// true if the contract issuer has submitted this item with the contract, false if the isser is asking for this item in the contract
    pub is_included:            bool,
    /// Unique ID for the item, used by the contract system
    pub record_id:              ItemId,
    /// Type ID for the item
    pub type_id:                TypeId,
    /// Unique ID for the item being sold. Not present if item is being requested by contract rather than sold with contract
    pub item_id:                Option<ItemId>,
    pub is_blueprint_copy:      Option<bool>,
    /// Material Efficiency Level of the blueprint
    pub material_efficiency:    Option<i32>,
    /// Time Efficiency Level of the blueprint
    pub time_efficiency:        Option<i32>,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original
    pub runs:                   Option<i32>,
}
