use sqlx::PgPool;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::industry_hub::IndustryHubUuid;

pub async fn update_industry_hubs(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
    update_info:        Vec<IndustryHubUuid>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionBeginError)?;

    sqlx::query!("
            DELETE FROM project_group_industry_hub
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::DeleteIndustryHubs(e, project_group_uuid))?;

    sqlx::query!("
            INSERT INTO project_group_industry_hub
            (
                project_group_id,
                industry_hub_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[]
            )
        ",
            *project_group_uuid,
            &update_info.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::UpdateIndustryHubs(e, project_group_uuid))?;

    transaction
        .commit()
        .await
        .map_err(ProjectGroupError::TransactionCommitError)
}

#[cfg(test)]
mod update_default_blueprint_overwrite_project_group_test {
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::update_industry_hubs(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
                vec![
                ],
            )
            .await;
        assert!(response.is_ok());
    }
}
