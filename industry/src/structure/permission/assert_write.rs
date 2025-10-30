use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::structure::StructureUuid;
use crate::structure::error::{StructureError, Result};

pub async fn assert_write_access(
    pool:           &PgPool,
    structure_uuid: StructureUuid,
    character_id:   CharacterId,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT id
            FROM structure pg
            WHERE id = $1
            AND owner = $2
        ",
            *structure_uuid,
            *character_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureError::FetchStructurePermission(e, structure_uuid))?;

    if result.is_none() {
        return Err(StructureError::Forbidden(structure_uuid, character_id));
    } else {
        Ok(())
    }
}
