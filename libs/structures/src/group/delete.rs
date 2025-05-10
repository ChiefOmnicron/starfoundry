use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, Result, StructureGroupUuid};

pub async fn delete(
    pool:          &PgPool,
    _character_id: CharacterId,
    group_uuid:    StructureGroupUuid,
) -> Result<StructureGroupUuid> {
    sqlx::query!(
        "
            DELETE FROM structure_group
            WHERE id = $1
        ",
            *group_uuid,
        )
        .execute(pool)
        .await
        .map_err(|e| Error::from(Error::DeleteGroup(e, group_uuid)))?;

    Ok(group_uuid)
}
