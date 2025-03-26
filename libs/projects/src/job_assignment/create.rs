use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use uuid::Uuid;

use crate::{CreateJobAssignment, Error, ProjectJobAssignmentUuid, Result};

pub async fn create(
    pool:           &PgPool,
    character_id:   CharacterId,
    job_assignment: CreateJobAssignment,
) -> Result<ProjectJobAssignmentUuid> {
    let assignment_uuid = Uuid::new_v4();

    let job_ids = sqlx::query!("
            SELECT DISTINCT(pj.id) AS job_id
            FROM projects p
            JOIN project_group_members pgm ON pgm.group_id = project_group_id
            JOIN project_jobs pj ON p.id = pj.project_id
            WHERE (
                pgm.character_id = $1 OR
                p.owner = $1
            )
            AND (
                pgm.projects = 'WRITE' OR
                pgm.projects = 'READ'
            )
            AND pj.id = ANY($2)
        ",
            *character_id,
            &job_assignment
                .job_ids
                .into_iter()
                .map(|x| *x)
                .collect::<Vec<_>>()
        )
        .fetch_all(pool)
        .await
        .map_err(Error::CreateJobAssignment)
        .map(|x| {
            x
                .into_iter()
                .map(|y| y.job_id)
                .collect::<Vec<_>>()
        })?;

    sqlx::query!("
            INSERT INTO project_job_assignments
            (
                id,
                job_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[]
            )
        ",
            assignment_uuid,
            &job_ids,
        )
        .execute(pool)
        .await
        .map(|_| assignment_uuid.into())
        .map_err(Error::CreateJobAssignment)
}
