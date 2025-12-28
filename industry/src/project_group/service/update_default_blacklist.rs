use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn update_default_blacklist(
    pool:                   &PgPool,
    project_group_uuid:     ProjectGroupUuid,
    type_ids:               Vec<TypeId>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionBeginError)?;

    sqlx::query!("
            DELETE FROM project_group_default_blacklist
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::DeleteGroupDefaults(e, project_group_uuid))?;

    sqlx::query!("
            INSERT INTO project_group_default_blacklist
            (
                project_group_id,
                type_id
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[]
            )
        ",
            *project_group_uuid,
            &type_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
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
mod update_default_blacklist_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::TypeId;
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
        let response = super::update_default_blacklist(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
                vec![TypeId(1)],
            )
            .await;
        assert!(response.is_ok());
    }
}
