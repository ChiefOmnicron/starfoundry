use serde::Deserialize;
use starfoundry_libs_structures::StructureUuid;
use utoipa::ToSchema;

use crate::ProjectUuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateJobDetectionAdd {
    pub delete_from_source:  bool,
    pub target_project_uuid: ProjectUuid,
    pub structure_id:        StructureUuid,
}
