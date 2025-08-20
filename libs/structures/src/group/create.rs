use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{CreateGroup, Error, Result, StructureGroupUuid};

pub async fn create(
    pool:            &PgPool,
    character_id:    CharacterId,
    structure_group: CreateGroup,
) -> Result<StructureGroupUuid> {
    let structure_group_id = sqlx::query!(
        "
            INSERT INTO structure_group
            (
                owner,
                name,
                structure_ids
            )
            VALUES (
                $1, $2, $3
            )
            RETURNING id
        ",
            *character_id,
            structure_group.name,
            &structure_group.structure_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_one(pool)
        .await
        .map_err(Error::CreateGroup)?
        .id
        .into();

    Ok(structure_group_id)
}
