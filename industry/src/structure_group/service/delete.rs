use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::structure_group::StructureGroupUuid;
use crate::structure_group::error::{Result, StructureGroupError};

pub async fn delete(
    pool:               &PgPool,
    character_id:       CharacterId,
    structure_group_id: StructureGroupUuid,
) -> Result<StructureGroupUuid> {
    sqlx::query!("
        DELETE FROM structure_group
        WHERE owner = $1
            AND id = $2
        RETURNING id
    ",
        *character_id,
        *structure_group_id,
    )
    .fetch_one(pool)
    .await
    .map(|x| StructureGroupUuid::new(x.id))
    .map_err(|e| StructureGroupError::DeleteStructureGroup(e, structure_group_id).into())
}

#[cfg(test)]
mod delete_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
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
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-100000000001").unwrap().into(),
            )
            .await;
        assert!(result.is_ok());
    }
}
