use serde::Deserialize;
use starfoundry_lib_types::ItemId;

/// Information about a location by [LocationId]
#[derive(Debug, Deserialize)]
pub struct AssetName {
    /// Id of the location id that maps to the name
    pub item_id: ItemId,
    /// Name of the location, for example a container or station
    pub name:    String,
}
