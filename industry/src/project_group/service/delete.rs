use sqlx::PgPool;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn delete(
    pool:     &PgPool,
    group_id: ProjectGroupUuid,
) -> Result<ProjectGroupUuid> {
    let has_project = sqlx::query!("
            SELECT 1 AS entry
            FROM project
            WHERE project_group_id = $1
        ",
            *group_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectGroupError::DeleteGroup(e, group_id))?;

    if has_project.is_some() {
        return Err(ProjectGroupError::ProjectIsAssignedToGroup);
    }

    sqlx::query!("
        DELETE FROM project_group
        WHERE id = $1
        RETURNING id
    ",
        *group_id,
    )
    .fetch_one(pool)
    .await
    .map(|x| ProjectGroupUuid::new(x.id))
    .map_err(|e| ProjectGroupError::DeleteGroup(e, group_id).into())
}

#[cfg(test)]
mod delete_project_group_test {
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures(
            path="../fixtures",
            scripts("base", "delete")
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::delete(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000010").unwrap().into(),
            )
            .await;
        assert!(result.is_ok());
    }

    #[sqlx::test(
        fixtures(
            path="../fixtures",
            scripts("base")
        ),
    )]
    async fn dont_delete_if_a_project_is_connected(
        pool: PgPool,
    ) {
        let result = super::delete(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await;
        assert!(result.is_err());
    }
}
