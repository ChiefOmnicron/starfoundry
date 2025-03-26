use serde::Serialize;
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::ProjectJobUuid;

#[derive(Debug, Serialize, ToSchema)]
#[serde(transparent)]
pub struct JobAssignment(HashMap<String, Vec<JobAssignmentEntry>>);

impl JobAssignment {
    pub fn new(entries: HashMap<String, Vec<JobAssignmentEntry>>) -> Self {
        Self(entries)
    }

    pub fn into_group(
        self,
    ) -> Vec<JobAssignmentGroup> {
        crate::job_assignment::sort_job_assignments_by_product_group(self.0)
    }

    pub fn into_inner(
        self
    ) -> HashMap<String, Vec<JobAssignmentEntry>> {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "category_id": 17,
        "group_id": 873,
        "item_name": "Neuolink Protection Cell",
        "job_id": null,
        "meta_group_id": null,
        "project_name": "My cool project",
        "runs": 1,
        "started": false,
        "structure_name": "1DQ1-A - Example Structure",
        "type_id": 57488
    })
)]
pub struct JobAssignmentEntry {
    pub job_id:         ProjectJobUuid,
    pub type_id:        TypeId,
    pub structure_name: String,
    pub project_name:   String,
    pub item_name:      String,
    pub runs:           i32,
    pub started:        bool,
    pub category_id:    i32,
    pub group_id:       i32,
    pub meta_group_id:  Option<i32>,
}

#[derive(Clone, Debug, Default, Serialize, ToSchema)]
#[schema(
    example = json!([{
        "header": "My cool project",
        "entries": [{
            "category_id": 17,
            "group_id": 873,
            "item_name": "Neuolink Protection Cell",
            "job_id": null,
            "meta_group_id": null,
            "project_name": "My cool project",
            "runs": 1,
            "started": false,
            "structure_name": "1DQ1-A - Example Structure",
            "type_id": 57488
        }]
    }])
)]
pub struct JobAssignmentGroup {
    pub header:  String,
    pub entries: Vec<JobAssignmentEntry>
}
