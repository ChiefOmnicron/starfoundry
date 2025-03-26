use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, Result, StructureDynamicGroupUuid};

pub async fn delete(
    pool:         &PgPool,
    character_id: CharacterId,
    group_uuid:   StructureDynamicGroupUuid,
) -> Result<StructureDynamicGroupUuid> {
    sqlx::query!(r#"
            DELETE FROM structure_dynamic_groups
            WHERE id = $1
              AND owner = $2
        "#,
            *group_uuid,
            *character_id,
        )
        .fetch_one(pool)
        .await
        .map(|_| group_uuid)
        .map_err(|e| Error::DeleteDynamicGroup(e, group_uuid))
}
