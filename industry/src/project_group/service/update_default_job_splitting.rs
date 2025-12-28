use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn update_default_job_splitting(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
    job_splitting:      Vec<UpdateProjectGroupDefaultJobSplitting>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionBeginError)?;

    sqlx::query!("
            DELETE FROM project_group_default_job_splitting_run
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::DeleteGroupDefaults(e, project_group_uuid))?;

    sqlx::query!("
            INSERT INTO project_group_default_job_splitting_run
            (
                project_group_id,
                type_id,
                max_runs
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            *project_group_uuid,
            &job_splitting.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
            &job_splitting.iter().map(|x| x.max_runs).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::UpdateGroupDefaults(e, project_group_uuid))?;

    transaction
        .commit()
        .await
        .map_err(ProjectGroupError::TransactionCommitError)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "material_efficiency": 10,
        "type_id": 23773
    })
)]
pub struct UpdateProjectGroupDefaultJobSplitting {
    pub type_id:  TypeId,
    pub max_runs: i32,
}

#[cfg(test)]
mod update_default_blueprint_overwrite_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::TypeId;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::project_group::service::UpdateProjectGroupDefaultJobSplitting;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::update_default_job_splitting(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
                vec![
                    UpdateProjectGroupDefaultJobSplitting {
                        type_id:  TypeId(1),
                        max_runs: 1,
                    }
                ],
            )
            .await;
        assert!(response.is_ok());
    }
}
