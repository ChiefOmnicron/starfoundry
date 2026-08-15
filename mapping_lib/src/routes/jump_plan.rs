use serde::{Deserialize, Serialize};
use starfoundry_lib_types::SystemId;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct JumpPlanEntry {
    pub system_id_start:    SystemId,
    pub system_id_end:      SystemId,
    pub distance:           i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateJumpPlan {
    pub system_start_id:            SystemId,
    pub system_end_id:              SystemId,
    pub max_distance_ly:            f32,

    #[serde(default)]
    pub blacklist_system_ids:       Vec<SystemId>,
    #[serde(default)]
    pub intermediate_system_ids:    Vec<SystemId>,
}
