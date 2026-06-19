use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use starfoundry_lib_types::{FittingId, TypeId};

use crate::LocationFlag;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool fit",
        "description": "You should check it out",
        "items": [{
            "quantity": 1,
            "type_id": 34489,
            "flag": "LoSlot0"
        }],
        "ship_type_id": 20185
    })
)]
pub struct EveFit {
    pub name:           String,
    pub description:    String,
    pub items:          Vec<EveFitItem>,
    pub ship_type_id:   TypeId,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "quantity": 1,
        "type_id": 34489,
        "flag": "LoSlot0"
    })
)]
pub struct EveFitItem {
    pub quantity:   i64,
    pub type_id:    TypeId,
    pub flag:       LocationFlag,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct EveFitResponse {
    fitting_id: FittingId,
}

impl Default for EveFitResponse {
    fn default() -> Self {
        Self {
            fitting_id: FittingId(0),
        }
    }
}
