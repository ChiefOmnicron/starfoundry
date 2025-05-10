use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, Result, StructureUuid, UpdateStructure};

pub async fn update(
    pool:           &PgPool,
    character_id:   CharacterId,
    structure_uuid: StructureUuid,
    structure:      UpdateStructure,
) -> Result<StructureUuid> {
    let result = sqlx::query!("
            UPDATE structure
            SET
                type_id   = $3,
                rigs      = $4,
                services  = $5,
                security  = $6,
                name      = $7,
                system_id = $8
            WHERE owner = $1
              AND id    = $2
        ",
            *character_id,
            *structure_uuid,
            structure.structure_type.into_i32(),
            &structure.rigs as _,
            &structure.services as _,
            structure.security as _,
            structure.name,
            *structure.system_id,
        )
        .execute(pool)
        .await
        .map_err(|e| Error::UpdateStructure(e, structure_uuid))?;

    if result.rows_affected() > 0 {
        Ok(structure_uuid)
    } else {
        Err(Error::StructureNotFound(structure_uuid))
    }
}
