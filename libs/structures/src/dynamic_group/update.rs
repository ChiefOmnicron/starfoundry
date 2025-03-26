use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, Result, StructureDynamicGroup, StructureDynamicGroupUuid};

pub async fn update(
    pool:         &PgPool,
    character_id: CharacterId,
    group_uuid:   StructureDynamicGroupUuid,
    group:        StructureDynamicGroup,
) -> Result<()> {
    sqlx::query!(r#"
            UPDATE structure_dynamic_groups
            SET
                name = $3,
                group_ids = $4
            WHERE id = $1
              AND owner = $2
        "#,
            *group_uuid,
            *character_id,
            group.name,
            &group.group_ids,
        )
        .fetch_one(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateDynamicGroup(e, group_uuid, group))
}
