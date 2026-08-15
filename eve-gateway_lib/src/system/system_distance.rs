use std::ops::Deref;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::System;
use starfoundry_lib_types::SystemId;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "system_start": {
            "region_id": 10000001,
            "constellation_id": 20000001,
            "system_id": 30000001,
            "region_name": "Derelik",
            "constellation_name": "San Matar",
            "system_name": "Tanoo",
            "security": 0.858324,
            "security_str": "HIGHSEC"
        },
        "system_end": {
            "region_id": 10000001,
            "constellation_id": 20000001,
            "system_id": 30000001,
            "region_name": "Derelik",
            "constellation_name": "San Matar",
            "system_name": "Tanoo",
            "security": 0.858324,
            "security_str": "HIGHSEC"
        },
        "distance_ly": 3.534
    })
)]
pub struct SystemDistance {
    pub system_start:   System,
    pub system_end:     System,
    pub distance_ly:    DistanceLy,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "system_start": 30000001,
        "system_end": 30000001,
        "distance_ly": 3.534
    })
)]
pub struct SystemDistanceMinimal {
    pub system_start:   SystemId,
    pub system_end:     SystemId,
    pub distance_ly:    DistanceLy,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct DistanceLy(f32);

impl From<f32> for DistanceLy {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Deref for DistanceLy {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
