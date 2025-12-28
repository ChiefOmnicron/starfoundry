use sqlx::PgPool;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::collections::HashSet;

use crate::{sort_by_job, Error, ProjectJobStatus, ProjectUuid, Result, StartableJob, StartableJobGroup, StartableJobEntry};

pub async fn startable_jobs(
    pool:         &PgPool,
    character_id: CharacterId,
    project_uuid: ProjectUuid,
) -> Result<StartableJob> {
    let mut jobs = sqlx::query!(
        r#"
            SELECT
                DISTINCT(pj.type_id) AS "type_id: TypeId",
                depends_on,

                i.name,
                i.volume,
                i.category_id,
                i.group_id,
                i.meta_group_id,

                pj.id,
                pj.runs,
                pj.status AS "status: ProjectJobStatus",
                pj.cost,
                pj.job_id,
                pj.structure_id
            FROM project_job pj
            JOIN blueprint_dependency bd ON pj.type_id = bd.ptype_id
            JOIN project p ON p.id = pj.project_id
            JOIN item i ON i.type_id = pj.type_id
            WHERE
                p.owner = $1 AND
                project_id = $2
        "#,
            *character_id,
            *project_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchStartableJobs(e, project_uuid))?
        .into_iter()
        .map(|x| StartableJobEntry {
            id:             x.id,
            type_id:        x.type_id.into(),
            runs:           x.runs,
            status:         x.status,
            cost:           x.cost,
            job_id:         x.job_id,
            structure_uuid: x.structure_id.into(),
            depends_on:     x.depends_on.into_iter().map(Into::into).collect::<Vec<_>>(),
            category_id:    x.category_id.into(),
            group_id:       x.group_id.into(),
            meta_group_id:  x.meta_group_id.map(|x| x.into()),
            item_name:      x.name,
        })
        .collect::<Vec<_>>();

    let job_ids = jobs
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();

    // Remove jobs that aren´t in our list of jobs
    for job in jobs.iter_mut() {
        job.depends_on = job
            .depends_on
            .iter_mut()
            .filter(|x| job_ids.contains(*x))
            .map(|x| *x)
            .collect::<Vec<_>>();
    }

    let mut buildable: Vec<StartableJobEntry> = Vec::new();

    let done = jobs
        .iter()
        .filter(|x| x.status == ProjectJobStatus::Done)
        .map(|x| x.type_id)
        .collect::<HashSet<_>>();

    let waiting_for_materials = jobs
        .iter()
        .filter(|x| x.status == ProjectJobStatus::WaitingForMaterials)
        .map(|x| x)
        .collect::<Vec<_>>();

    for waiting_for_material in waiting_for_materials {
        if waiting_for_material
            .depends_on
            .iter()
            .all(|x| done.contains(x)) {

            buildable.push(waiting_for_material.clone());
        }
    }

    Ok(StartableJob::new(buildable))
}

sort_by_job!(sort_startable_jobs_by_group_id, StartableJobEntry, StartableJobGroup);
