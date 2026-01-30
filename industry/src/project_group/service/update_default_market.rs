use sqlx::PgPool;
use starfoundry_lib_industry::StructureUuid;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn update_default_market(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
    structures:         Vec<StructureUuid>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionBeginError)?;

    sqlx::query!("
            DELETE FROM project_group_default_market
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::DeleteGroupDefaults(e, project_group_uuid))?;

    sqlx::query!("
            INSERT INTO project_group_default_market
            (
                project_group_id,
                structure_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[]
            )
        ",
            *project_group_uuid,
            &structures.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::UpdateGroupDefaults(e, project_group_uuid))?;

    transaction
        .commit()
        .await
        .map_err(ProjectGroupError::TransactionCommitError)
}


#[cfg(test)]
mod update_default_blueprint_overwrite_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_industry::StructureUuid;
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
        let response = super::update_default_market(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
                vec![StructureUuid::from(Uuid::now_v7())],
            )
            .await;
        assert!(response.is_ok());
    }
}
