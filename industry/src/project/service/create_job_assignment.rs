use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::project::error::{ProjectError, Result};
use crate::project::{ProjectAssignmentUuid, ProjectUuid};
use crate::project::service::ProjectJobUuid;
use uuid::Uuid;

pub async fn create_job_assignment(
    pool:         &PgPool,
    project_info: Vec<CreateProjectJobAssignment>,
) -> Result<ProjectAssignmentUuid> {
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
        .map_err(ProjectError::Create)?;

    Ok(id.into())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectJobAssignment {
    project_id: ProjectUuid,
    job_id:     ProjectJobUuid,
}
