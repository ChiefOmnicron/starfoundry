use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, Result, StructureGroup, StructureGroupUuid};

pub async fn fetch(
    pool:         &PgPool,
    character_id: CharacterId,
    group_uuid:   StructureGroupUuid,
) -> Result<StructureGroup> {
    let structure_ids = sqlx::query!("
            SELECT structure_id
            FROM structure_group_structure
            WHERE structure_group_id = $1
        ",
            *group_uuid,
        )
        .fetch_all(pool)
        .await
        .unwrap();

    sqlx::query!(
        "
            SELECT
                id,
                name
            FROM structure_group
            WHERE owner = $1
            AND id = $2
        ",
            *character_id,
            *group_uuid,
        )
        .fetch_one(pool)
        .await
        .map(|x| {
            let structure_ids = structure_ids
                .iter()
                .map(|y| y.structure_id)
                .map(|y| y.into())
                .collect::<Vec<_>>();

            StructureGroup {
                id:            x.id.into(),
                name:          x.name,
                structure_ids: structure_ids,
            }
        })
        .map_err(|e| Error::FetchGroup(e, group_uuid))
}

