use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, Result, StructureUuid};

pub async fn delete(
    pool:           &PgPool,
    character_id:   CharacterId,
    structure_uuid: StructureUuid,
) -> Result<()> {
    let result = sqlx::query!(r#"
            DELETE FROM structures
            WHERE owner = $1
            AND id = $2
        "#,
            *character_id,
            *structure_uuid,
        )
        .execute(pool)
        .await
        .map_err(|e| Error::DeleteStructure(e, structure_uuid))?;

    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(Error::StructureNotFound(structure_uuid))
    }
}
