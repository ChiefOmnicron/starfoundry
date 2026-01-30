use sqlx::PgPool;
use starfoundry_lib_industry::StructureUuid;
use starfoundry_lib_types::CharacterId;

use crate::structure::StructureError;
use crate::structure::error::Result;

pub async fn delete(
    pool:         &PgPool,
    character_id: CharacterId,
    structure_id: StructureUuid,
) -> Result<StructureUuid> {
    sqlx::query!("
        DELETE FROM structure
        WHERE owner = $1
            AND id = $2
        RETURNING id
    ",
        *character_id,
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
                Uuid::from_str("00000000-0000-0000-0000-000000000003").unwrap().into(),
            )
            .await;
        assert!(result.is_ok());
    }
}
