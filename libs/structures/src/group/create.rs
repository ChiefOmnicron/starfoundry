use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{CreateGroup, Error, Result, StructureGroupUuid};
use uuid::Uuid;

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
                name
            )
            VALUES (
                $1, $2
            )
            RETURNING id
        ",
            *character_id,
            structure_group.name,
        )
        .fetch_one(pool)
        .await
        .map_err(Error::CreateGroup)?
        .id
        .into();

    sqlx::query!("
            INSERT INTO structure_group_structure (structure_group_id, structure_id)
            SELECT $1, * FROM UNNEST($2::UUID[])
        ",
            structure_group_id,
            &structure_group.structure_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .execute(pool)
        .await
        .unwrap();

    Ok(Uuid::new_v4().into())
}
