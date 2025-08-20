use serde::Deserialize;
use starfoundry_lib_types::{CharacterId, JobId};
use utoipa::ToSchema;

use crate::{ProjectJobStatus, ProjectUuid};

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateJob {
    pub id:     ProjectUuid,
    pub status: ProjectJobStatus,

    pub cost:         Option<f32>,
    pub job_id:       Option<JobId>,
    pub character_id: Option<CharacterId>,
}
