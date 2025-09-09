use serde::Serialize;
use starfoundry_lib_structures::StructureUuid;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{IndustryActivity, ProjectJobStatus};

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "activity": "MANUFACTURING",
        "cost": 1337,
        "delivered": false,
        "end_date": "2024-12-29T20:20:42Z",
        "id": "3b51c53e-d266-4d2d-b8b5-0a7325e53814",
        "job_id": 588054964,
        "runs": 10,
        "status": "IN_PROGRESS",
        "structure_name": "1DQ1-A - Example Station",
        "structure_uuid": "f18015ef-7be9-4f46-8bda-95f25d4f24b7",
        "type_id": 73790
    })
)]
pub struct ActiveJob {
    pub id:             Uuid,
    pub type_id:        TypeId,
    pub runs:           i32,
    pub status:         ProjectJobStatus,
    pub structure_uuid: StructureUuid,
    pub cost:           Option<f64>,
    pub job_id:         Option<i32>,
    pub delivered:      bool,
    pub end_date:       String,
    pub activity:       IndustryActivity,
    pub structure_name: String,
}
