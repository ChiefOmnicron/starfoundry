use serde::Serialize;
use starfoundry_libs_types::{CategoryId, GroupId, TypeId};
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
        let mut grouped_jobs = HashMap::new();

        for (group, entries) in self.0 {
            let job_groups = crate::job_assignment::sort_job_assignments_jobs(entries);
            grouped_jobs.insert(group, job_groups);
        }

        crate::job_assignment::sort_job_assignments_by_product_group(grouped_jobs)
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
        "type_id": 57488,
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
    pub category_id:    CategoryId,
    pub group_id:       GroupId,
    pub meta_group_id:  Option<GroupId>,
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
    pub entries: Vec<JobAssignmentGroupEntry>
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct JobAssignmentGroupEntry {
    pub header:  String,
    pub entries: Vec<JobAssignmentEntry>
}
