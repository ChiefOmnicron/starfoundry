use sqlx::PgPool;

use crate::{Error, ProjectGroupUuid, Result};
use super::ProjectGroupDefault;

pub async fn update_default(
    pool:     &PgPool,
    group_id: ProjectGroupUuid,
    defaults: ProjectGroupDefault,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionBeginError)?;

    sqlx::query!("
            DELETE FROM project_group_default_blacklist
            WHERE project_group_id = $1
        ",
            *group_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::UpdateGroupDefaults(e, group_id))?;

    sqlx::query!("
            DELETE FROM project_group_default_markets
            WHERE project_group_id = $1
        ",
            *group_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::UpdateGroupDefaults(e, group_id))?;

    sqlx::query!("
            INSERT INTO project_group_default_markets (
                project_group_id,
                structure_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[]
            )
        ",
            *group_id,
            &defaults.markets.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::UpdateGroupDefaults(e, group_id))?;

    sqlx::query!("
            INSERT INTO project_group_default_blacklist (
                project_group_id,
                type_id
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[]
            )
        ",
            *group_id,
            &defaults.blacklist.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::UpdateGroupDefaults(e, group_id))?;

    transaction
        .commit()
        .await
        .map_err(Error::TransactionCommitError)?;

    Ok(())
}
