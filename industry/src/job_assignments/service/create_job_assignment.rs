use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_industry::{ProjectJobUuid, ProjectUuid};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::job_assignments::error::{JobAssignmentError, Result};
use crate::job_assignments::JobAssignmentUuid;

pub async fn create_job_assignment(
    pool:         &PgPool,
    project_info: Vec<CreateProjectJobAssignment>,
) -> Result<JobAssignmentUuid> {
    let id = Uuid::now_v7();
    let mut project_ids = Vec::new();
    let mut job_ids = Vec::new();

    project_info
        .iter()
        .for_each(|x| {
            project_ids.push(*x.project_id);
            job_ids.push(*x.job_id);
        });

    sqlx::query!(r#"
            INSERT INTO project_job_assignment
            (
                id,
                project_id,
                job_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[],
                $3::UUID[]
            )
        "#,
            id,
            &project_ids,
            &job_ids,
        )
        .execute(pool)
        .await
        .map_err(JobAssignmentError::Create)?;

    Ok(id.into())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectJobAssignment {
    project_id: ProjectUuid,
    job_id:     ProjectJobUuid,
}
