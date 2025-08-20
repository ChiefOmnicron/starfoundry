use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{CreateStructure, Error, Result, StructureUuid};

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    structure:    CreateStructure,
) -> Result<StructureUuid> {
    let structure_id: i32 = structure.structure_type.into();

    sqlx::query!("
            INSERT INTO structure
            (
                owner,
                type_id,
                rigs,
                services,
                security,
                name,
                system_id,
                structure_id
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
        ",
            *character_id,
            structure_id,
            &structure.rigs as _,
            &structure.services as _,
            structure.security as _,
            structure.name,
            *structure.system_id,
            structure.structure_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| StructureUuid::new(x.id))
        .map_err(|e| Error::Create(e))
}
