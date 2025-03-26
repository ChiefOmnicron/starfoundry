use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{CreateJobAssignment, Error, JobAssignment, ProjectJobAssignmentUuid, ProjectJobUuid, Result};

pub struct ProjectJobAssignmentService(ProjectJobAssignmentUuid);

impl ProjectJobAssignmentService {
    pub fn new(
        project_uuid: ProjectJobAssignmentUuid,
    ) -> Self {
        ProjectJobAssignmentService(project_uuid)
    }

    pub async fn assert_exists(
        &self,
        pool: &PgPool,
    ) -> Result<()> {
        let project = sqlx::query!("
                SELECT id
                FROM project_job_assignments
                WHERE id = $1
            ",
                *self.0,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchProjectJobAssignments(e, self.0))?;

        if project.is_some() {
            Ok(())
        } else {
            Err(Error::ProjectJobAssignmentNotFound(self.0))
        }
    }

    pub async fn fetch(
        &self,
        pool:            &PgPool,
    ) -> Result<JobAssignment> {
        self.assert_exists(pool).await?;

        crate::job_assignment::fetch(
                pool,
                self.0,
            )
            .await
    }

    pub async fn create(
        pool:           &PgPool,
        character_id:   CharacterId,
        job_assignment: CreateJobAssignment,
    ) -> Result<ProjectJobAssignmentUuid> {
        crate::job_assignment::create(
                pool,
                character_id,
                job_assignment,
            )
            .await
    }

    pub async fn update_job_state(
        &self,
        pool:            &PgPool,
        job_uuid:        ProjectJobUuid,
    ) -> Result<()> {
        self.assert_exists(pool).await?;

        crate::job_assignment::update_job_state(
                pool,
                self.0,
                job_uuid,
            )
            .await
    }
}
