use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ProjectJobUuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateJobAssignment {
    pub job_ids: Vec<ProjectJobUuid>,
}
