use serde::Serialize;
use starfoundry_lib_types::{JobId, TypeId};
use utoipa::ToSchema;

use crate::ProjectUuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct JobDetection {
    pub project_uuid: Option<ProjectUuid>,
    pub type_id:      TypeId,
    pub runs:         i32,
    pub end_date:     String,
    pub job_id:       JobId,
}
