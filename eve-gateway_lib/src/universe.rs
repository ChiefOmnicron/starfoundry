use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{ConstellationId, RegionId, SystemId};
use utoipa::ToSchema;

// TODO: move me when a better location is found
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "region_id": 10000001,
        "constellation_id": 20000001,
        "system_id": 30000001,
        "region_name": "Derelik",
        "constellation_name": "San Matar",
        "system_name": "Tanoo",
        "security": 0.858324
    })
)]
pub struct System {
    pub region_id:          RegionId,
    pub region_name:        String,
    pub constellation_id:   ConstellationId,
    pub constellation_name: String,
    pub system_id:          SystemId,
    pub system_name:        String,
    pub security:           f32,
    pub security_str:       String,
}
