use serde::Serialize;
use starfoundry_lib_structures::StructureUuid;
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::ProjectJobStatus;

#[derive(Debug, Serialize, ToSchema)]
#[serde(transparent)]
pub struct Job(Vec<JobEntry>);

impl Job {
    pub fn new(entries: Vec<JobEntry>) -> Self {
        Self(entries)
    }

    pub fn into_group(
        self,
    ) -> Vec<JobGroup> {
        crate::job::sort_jobs_by_group_id(self.0)
    }

    pub fn into_inner(
        self
    ) -> Vec<JobEntry> {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "category_id": 17,
        "cost": 1337,
        "group_id": 873,
        "id": "e953781d-dc45-42b8-92e3-f6b07afaca6e",
        "item_name": "Neuolink Protection Cell",
        "job_id": null,
        "meta_group_id": null,
        "runs": 1,
        "status": "WAITING_FOR_MATERIALS",
        "structure_uuid": "1f39057c-c35b-4578-8309-50e84db3ee93",
        "type_id": 57488
    })
)]
pub struct JobEntry {
    pub id:             Uuid,
    pub type_id:        TypeId,
    pub runs:           i32,
    pub status:         ProjectJobStatus,
    pub structure_uuid: StructureUuid,
    pub cost:           Option<f64>,
    pub job_id:         Option<i32>,
    pub item_name:      String,
    pub category_id:    CategoryId,
    pub group_id:       GroupId,
    pub meta_group_id:  Option<GroupId>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!([{
        "header": "CAPITAL_CONSTRUCTION_COMPONENTS",
        "entries": [{
            "category_id": 17,
            "cost": 1337,
            "group_id": 873,
            "id": "e953781d-dc45-42b8-92e3-f6b07afaca6e",
            "item_name": "Neuolink Protection Cell",
            "job_id": null,
            "meta_group_id": null,
            "runs": 1,
            "status": "WAITING_FOR_MATERIALS",
            "structure_uuid": "1f39057c-c35b-4578-8309-50e84db3ee93",
            "type_id": 57488
        }]
    }])
)]
pub struct JobGroup {
    pub header:  String,
    pub entries: Vec<JobEntry>,
}
