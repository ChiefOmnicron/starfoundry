use sqlx::PgPool;

use crate::{ActiveJob, Error, ProjectUuid, ProjectJobStatus, IndustryActivity, Result};

pub async fn active_jobs(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Vec<ActiveJob>> {
    sqlx::query!(
        r#"
            SELECT
                DISTINCT(pj.job_id),

                pj.id,
                pj.runs,
                status AS "status: ProjectJobStatus",
                pj.cost,
                pj.structure_id,
                pj.type_id,
                ij.is_delivered,
                ij.end_date,
                ij.activity AS "activity: IndustryActivity",
                s.name AS structure_name
            FROM project_jobs pj
            JOIN industry_jobs ij ON ij.job_id = pj.job_id
            JOIN structures s ON (s.structure_id = ij.facility_id AND s.id = pj.structure_id)
            WHERE
                pj.project_id = $1 AND
                (pj.status = 'BUILDING' OR ij.is_delivered = false)
            ORDER BY end_date ASC
        "#,
            *project_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchActiveJobs(e, project_uuid))
        .map(|x| {
            x.into_iter()
                .map(|x| ActiveJob {
                    id:             x.id,
                    type_id:        x.type_id.into(),
                    runs:           x.runs,
                    status:         x.status,
                    cost:           x.cost,
                    job_id:         x.job_id,
                    structure_uuid:   x.structure_id.into(),
                    delivered:      x.is_delivered,
                    end_date:       x.end_date,
                    activity:       x.activity,
                    structure_name: x.structure_name,
                })
                .collect::<Vec<_>>()
        })
}
