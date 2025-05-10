use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, Result, StructureDynamicGroup, StructureDynamicGroupUuid};

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    group:        StructureDynamicGroup,
) -> Result<StructureDynamicGroupUuid> {
    let id = sqlx::query!(r#"
            INSERT INTO structure_dynamic_group
            (
                name,
                group_ids,
                owner
            )
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
            group.name,
            &group.group_ids,
            *character_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::CreateDynamicGroup(e, group))?
        .id;
    Ok(id.into())
}
