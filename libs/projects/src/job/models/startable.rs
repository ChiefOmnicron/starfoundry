use serde::Serialize;
use starfoundry_libs_structures::StructureUuid;
use starfoundry_libs_types::{CategoryId, GroupId, TypeId};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::ProjectJobStatus;

#[derive(Debug, Serialize, ToSchema)]
#[serde(transparent)]
pub struct StartableJob(Vec<StartableJobEntry>);

impl StartableJob {
    pub fn new(entries: Vec<StartableJobEntry>) -> Self {
        Self(entries)
    }

    pub fn into_group(
        self,
    ) -> Vec<StartableJobGroup> {
        crate::job::sort_startable_jobs_by_group_id(self.0)
    }

    pub fn into_inner(
        self
    ) -> Vec<StartableJobEntry> {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "category_id": 4,
        "cost": null,
        "depends_on": [
            16657,
            33337
        ],
        "group_id": 429,
        "id": "ad69a405-48cf-45ed-947a-16476d25d132",
        "item_name": "Terahertz Metamaterials",
        "job_id": null,
        "meta_group_id": null,
        "runs": 5,
        "status": "WAITING_FOR_MATERIALS",
        "structure_uuid": "5f3f3120-82be-44a6-91a9-dea011c03186",
        "type_id": 33360
    })
)]
pub struct StartableJobEntry {
    pub depends_on:   Vec<TypeId>,

    pub id:             Uuid,
    pub type_id:        TypeId,
    pub runs:           i32,
    pub status:         ProjectJobStatus,
    pub structure_uuid: StructureUuid,
    pub cost:           Option<f64>,
    pub job_id:         Option<i32>,
    pub group_id:       GroupId,
    pub meta_group_id:  Option<GroupId>,
    pub category_id:    CategoryId,
    pub item_name:      String,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!([{
        "header": "COMPOSITE_REACTIONS",
        "entries": [{
            "category_id": 4,
            "cost": null,
            "depends_on": [
                16657,
                33337
            ],
            "group_id": 429,
            "id": "ad69a405-48cf-45ed-947a-16476d25d132",
            "item_name": "Terahertz Metamaterials",
            "job_id": null,
            "meta_group_id": null,
            "runs": 5,
            "status": "WAITING_FOR_MATERIALS",
            "structure_uuid": "5f3f3120-82be-44a6-91a9-dea011c03186",
            "type_id": 33360
        }]
    }])
)]
pub struct StartableJobGroup {
    pub header:  String,
    pub entries: Vec<StartableJobEntry>,
}
