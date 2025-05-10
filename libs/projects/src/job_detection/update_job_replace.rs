use sqlx::PgPool;
use starfoundry_libs_types::{CharacterId, JobId};

use crate::{Error, ProjectService, Result, UpdateJobDetectionReplace};

pub async fn update_job_replace(
    pool:         &PgPool,
    character_id: CharacterId,
    job_id:       JobId,
    update:       UpdateJobDetectionReplace,
) -> Result<()> {
    let project = ProjectService::new(update.target_project_uuid);
    project.assert_exists(pool).await?;
    project.assert_write_access(pool, character_id).await?;

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

    sqlx::query!("
            DELETE FROM project_job
            WHERE id = ANY($1::UUID[])
        ",
            &update.job_uuids.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteProjectJob)?;

    let job_info = sqlx::query!("
            SELECT
                type_id,
                runs,
                cost
            FROM industry_job ij
            JOIN job_detection_log jdl ON jdl.job_id = ij.job_id
            WHERE ij.job_id = $1
        ",
            *job_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::UpdateIndustryJobByJobId(e, job_id))?;

    // insert it into the new project
    // setting it to BUILDING is a safe bet. The job will only be shown if it
    // was started, and even if it's done, the job detection will recognize it
    // in the next cycle and just update it to 'DONE'
    sqlx::query!("
            INSERT INTO project_job (
                project_id,
                type_id,
                runs,
                cost,
                job_id,
                structure_id,
                status
            )
            VALUES ($1, $2, $3, $4, $5, $6, 'BUILDING')
        ",
            *update.target_project_uuid,
            job_info.type_id,
            job_info.runs,
            job_info.cost,
            *job_id,
            *update.structure_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertProjectJob)?;

    // update in job detection log
    sqlx::query!("
            UPDATE job_detection_log
            SET project_id = $1
            WHERE job_id = $2
        ",
            *update.target_project_uuid,
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

