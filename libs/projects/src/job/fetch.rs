use sqlx::PgPool;

use crate::{sort_by_job, Error, FetchJobFilter, Job, JobEntry, JobGroup, ProjectJobStatus, ProjectUuid, Result};

pub async fn fetch(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    filter:       FetchJobFilter,
) -> Result<Job> {
    sqlx::query!(
        r#"
            SELECT
                i.*,

                id,
                runs,
                status AS "status: ProjectJobStatus",
                cost,
                job_id,
                structure_id
            FROM project_job pj
            JOIN item i ON i.type_id = pj.type_id
            WHERE pj.project_id = $1
            AND (
                (
                    NOT (pj.type_id = $2::INTEGER) IS FALSE
                )
            )
        "#,
            *project_uuid,
            filter.type_id.map(|x| *x),
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchJobs(e, project_uuid))
        .map(|rows| {
            rows
                .into_iter()
                .map(|x| JobEntry {
                    id:             x.id,
                    type_id:        x.type_id.into(),
                    runs:           x.runs,
                    status:         x.status,
                    cost:           x.cost,
                    job_id:         x.job_id,
                    structure_uuid: x.structure_id.into(),
                    item_name:      x.name,
                    category_id:    x.category_id.into(),
                    group_id:       x.group_id.into(),
                    meta_group_id:  x.meta_group_id.map(|x| x.into()),
                })
                .collect::<Vec<_>>()
        })
        .map(|x| Job::new(x))
}

sort_by_job!(sort_jobs_by_group_id, JobEntry, JobGroup);
