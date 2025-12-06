use sqlx::PgPool;
use starfoundry_libs_types::{CharacterId, JobId};

use crate::{Error, JobDetection, Result, UpdateJobDetectionAdd, UpdateJobDetectionDelete, UpdateJobDetectionReplace};

pub struct JobDetectionService;

impl JobDetectionService {
    pub async fn fetch(
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<JobDetection>> {
        if character_id != CharacterId(2117441999) {
            return Err(Error::NotEistonen);
        }

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
        if character_id != CharacterId(2117441999) {
            return Err(Error::NotEistonen);
        }

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
        if character_id != CharacterId(2117441999) {
            return Err(Error::NotEistonen);
        }

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
        if character_id != CharacterId(2117441999) {
            return Err(Error::NotEistonen);
        }

        crate::job_detection::update_job_replace(
                pool,
                character_id,
                job_id,
                update,
            )
            .await
    }
}
