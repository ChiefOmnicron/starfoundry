use sqlx::PgPool;

use crate::{Error, JobDetection, Result};

pub async fn fetch(
    pool: &PgPool,
) -> Result<Vec<JobDetection>> {
    sqlx::query!("
            SELECT
                jdl.type_id,
                jdl.project_id,
                ij.runs,
                ij.end_date,
                ij.job_id
            FROM job_detection_logs jdl
            JOIN industry_jobs ij ON ij.job_id = jdl.job_id
            WHERE is_delivered = false
            ORDER BY end_date ASC
        ")
        .fetch_all(pool)
        .await
        .map_err(Error::FetchJobDetection)
        .map(|x| {
            x.into_iter()
                .map(|y| JobDetection {
                    project_uuid: y.project_id.map(Into::into),
                    runs:         y.runs,
                    type_id:      y.type_id.into(),
                    end_date:     y.end_date,
                    job_id:       y.job_id.into(),
                })
                .collect::<Vec<_>>()
        })
}
