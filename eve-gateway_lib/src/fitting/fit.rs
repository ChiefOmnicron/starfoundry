use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use starfoundry_lib_types::{FittingId, TypeId};

use crate::LocationFlag;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct EveFit {
    name:           String,
    description:    String,
    items:          Vec<EveFitItem>,
    ship_type_id:   TypeId,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct EveFitItem {
    quantity:   i64,
    type_id:    TypeId,
    flag:       LocationFlag,
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
