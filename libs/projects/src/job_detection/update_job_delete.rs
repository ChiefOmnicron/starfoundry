use sqlx::PgPool;
use starfoundry_lib_types::{CharacterId, JobId};

use crate::{Error, ProjectService, Result, UpdateJobDetectionDelete};

pub async fn update_job_delete(
    pool:         &PgPool,
    character_id: CharacterId,
    job_id:       JobId,
    update:       UpdateJobDetectionDelete,
) -> Result<()> {
    // check if the job id is already assigned
    let project_id = sqlx::query!("
            SELECT project_id
            FROM project_job
            WHERE job_id = $1
        ",
            *job_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchProjectJobByJobId(e, job_id))?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionBeginError)?;

    // if it already exists, remove it
    if let Some(x) = project_id {
        let project = ProjectService::new(x.project_id.into());
        project.assert_exists(pool).await?;
        project.assert_write_access(pool, character_id).await?;

        if update.delete_from_source {
            sqlx::query!("
                    DELETE FROM project_job
                    WHERE project_id = $1
                    AND job_id = $2
                ",
                    x.project_id,
                    *job_id,
                )
                .execute(&mut *transaction)
                .await
                .map_err(|e| Error::DeleteFromProjectJobByJobId(e, job_id))?;
        } else {
            sqlx::query!("
                    UPDATE project_job
                    SET
                        status = 'WAITING_FOR_MATERIALS',
                        job_id = NULL,
                        cost = NULL
                    WHERE project_id = $1
                    AND job_id = $2
                ",
                    x.project_id,
                    *job_id,
                )
                .execute(&mut *transaction)
                .await
                .map_err(|e| Error::UpdateProjectJobByJobId(e, job_id))?;
        }
    }

    if update.ignore {
        sqlx::query!("
                UPDATE industry_job
                SET ignore = true
                WHERE job_id = $1
            ",
                *job_id
            )
            .execute(&mut *transaction)
            .await
            .map_err(|e| Error::UpdateIndustryJobByJobId(e, job_id))?;
    }

    // update in job detection log
    sqlx::query!("
            UPDATE job_detection_log
            SET project_id = NULL
            WHERE job_id = $1
        ",
            *job_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::UpdateJobDetection)?;

    transaction
        .commit()
        .await
        .map_err(Error::TransactionCommitError)?;

    Ok(())
}

