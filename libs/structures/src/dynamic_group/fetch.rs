use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, Result, StructureDynamicGroup, StructureDynamicGroupUuid};

pub async fn fetch(
    pool:         &PgPool,
    character_id: CharacterId,
    group_uuid:   StructureDynamicGroupUuid,
) -> Result<Option<StructureDynamicGroup>> {
    let group = sqlx::query!(r#"
            SELECT
                id,
                name,
                group_ids
            FROM structure_dynamic_group
            WHERE owner = $1
              AND id = $2
        "#,
            *character_id,
            *group_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::GroupIdById(e, group_uuid))?
        .map(|x| StructureDynamicGroup {
            id:        Some(x.id.into()),
            name:      x.name,

            group_ids: x.group_ids,
        });
    Ok(group)
}
