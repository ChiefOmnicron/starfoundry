use sqlx::PgPool;
use std::collections::HashMap;

use crate::{sort_by_job, sort_by_product_group, Error, JobAssignment, JobAssignmentEntry, JobAssignmentGroupEntry, JobAssignmentGroup, ProjectJobAssignmentUuid, Result};

pub async fn fetch(
    pool:          &PgPool,
    assignment_id: ProjectJobAssignmentUuid,
) -> Result<JobAssignment> {
    let assignments = sqlx::query!("
            SELECT
                pja.job_id,
                pja.started,
                pj.type_id,
                pj.runs,
                s.name AS structure_name,
                p.name AS project_name,
                i.name AS item_name,
                i.category_id,
                i.group_id,
                i.meta_group_id
            FROM project_job_assignment pja
            JOIN project_job pj ON pj.id = pja.job_id
            JOIN project p ON p.id = pj.project_id
            JOIN item i ON i.type_id = pj.type_id
            JOIN structure s ON s.id = pj.structure_id
            WHERE pja.id = $1
            ORDER BY i.name
        ",
            *assignment_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchProjectJobAssignments(e, assignment_id))?
        .into_iter()
        .map(|x| {
            JobAssignmentEntry {
                job_id:         x.job_id.into(),
                structure_name: x.structure_name,
                type_id:        x.type_id.into(),
                project_name:   x.project_name,
                item_name:      x.item_name,
                runs:           x.runs,
                started:        x.started,
                category_id:    x.category_id.into(),
                group_id:       x.group_id.into(),
                meta_group_id:  x.meta_group_id.map(Into::into)
            }
        })
        .collect::<Vec<_>>();

    let mut mapped = HashMap::new();
    for assignment in assignments {
        mapped
            .entry(assignment.project_name.clone())
            .and_modify(|x: &mut Vec<JobAssignmentEntry>| x.push(assignment.clone()))
            .or_insert(vec![assignment]);
    }

    Ok(JobAssignment::new(mapped))
}

sort_by_job!(sort_job_assignments_jobs, JobAssignmentEntry, JobAssignmentGroupEntry);
sort_by_product_group!(sort_job_assignments_by_product_group, JobAssignmentGroupEntry, JobAssignmentGroup);
