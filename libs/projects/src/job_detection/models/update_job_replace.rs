use serde::Deserialize;
use starfoundry_lib_structures::StructureUuid;
use utoipa::ToSchema;

use crate::{ProjectJobUuid, ProjectUuid};

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateJobDetectionReplace {
    pub delete_from_source:  bool,
    pub job_uuids:           Vec<ProjectJobUuid>,
    pub structure_id:        StructureUuid,
    pub target_project_uuid: ProjectUuid,
}
