use sqlx::PgPool;

use crate::structure::StructureUuid;
use crate::structure::error::{StructureError, Result};

pub async fn assert_exists(
    pool:           &PgPool,
    structure_uuid: StructureUuid,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT id
            FROM structure
            WHERE id = $1
        ",
            *structure_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureError::FetchStructurePermission(e, structure_uuid))?;

    if result.is_some() {
        Ok(())
    } else {
        Err(StructureError::NotFound(structure_uuid))
    }
}
