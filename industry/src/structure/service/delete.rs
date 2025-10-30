use sqlx::PgPool;

use crate::structure::{StructureError, StructureUuid};
use crate::structure::error::Result;

pub async fn delete(
    pool:         &PgPool,
    structure_id: StructureUuid,
) -> Result<StructureUuid> {
    sqlx::query!("
        DELETE FROM structure
        WHERE id = $1
        RETURNING id
    ",
        *structure_id,
    )
    .fetch_one(pool)
    .await
    .map(|x| StructureUuid::new(x.id))
    .map_err(|e| StructureError::DeleteStructure(e, structure_id).into())
}

#[cfg(test)]
mod delete_project_group_test {
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures(
            path="../fixtures",
            scripts("base")
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::delete(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000003").unwrap().into(),
            )
            .await;
        assert!(result.is_ok());
    }
}
