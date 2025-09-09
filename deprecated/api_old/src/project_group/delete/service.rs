use sqlx::PgPool;

use crate::project_group::error::{Error, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn delete(
    pool:     &PgPool,
    group_id: ProjectGroupUuid,
) -> Result<ProjectGroupUuid> {
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
    .map_err(|e| Error::DeleteGroup(e, group_id).into())
}

#[cfg(test)]
mod delete_project_group_test {
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures(
            path="../fixtures",
            scripts("delete")
        ),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path(
        connection: PoolConnection<Postgres>,
    ) {
        let result = super::delete(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await;
        assert!(result.is_ok());
    }
}
