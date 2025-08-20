use sqlx::PgPool;
use starfoundry_lib_types::{CharacterId, JobId};

use crate::{JobDetection, Result, UpdateJobDetectionAdd, UpdateJobDetectionDelete, UpdateJobDetectionReplace};

pub struct JobDetectionService;

impl JobDetectionService {
    pub async fn fetch(
        pool: &PgPool,
    ) -> Result<Vec<JobDetection>> {
        crate::job_detection::fetch(
                pool,
            )
            .await
    }

    pub async fn update_job_add(
        pool:         &PgPool,
        character_id: CharacterId,
        job_id:       JobId,
        update:       UpdateJobDetectionAdd,
    ) -> Result<()> {
        crate::job_detection::update_job_add(
                pool,
                character_id,
                job_id,
                update,
            )
            .await
    }

    pub async fn update_job_delete(
        pool:         &PgPool,
        character_id: CharacterId,
        job_id:       JobId,
        update:       UpdateJobDetectionDelete,
    ) -> Result<()> {
        crate::job_detection::update_job_delete(
                pool,
                character_id,
                job_id,
                update,
            )
            .await
    }

    pub async fn update_job_replace(
        pool:         &PgPool,
        character_id: CharacterId,
        job_id:       JobId,
        update:       UpdateJobDetectionReplace,
    ) -> Result<()> {
        crate::job_detection::update_job_replace(
                pool,
                character_id,
                job_id,
                update,
            )
            .await
    }
}
