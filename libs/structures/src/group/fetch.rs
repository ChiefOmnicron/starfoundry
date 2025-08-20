use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, Result, StructureGroup, StructureGroupUuid};

pub async fn fetch(
    pool:         &PgPool,
    character_id: CharacterId,
    group_uuid:   StructureGroupUuid,
) -> Result<StructureGroup> {
    sqlx::query!(
        "
            SELECT
                id,
                name,
                structure_ids
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
            let structure_ids = x.structure_ids
                .iter()
                .map(|y| (*y).into())
                .collect::<Vec<_>>();

            StructureGroup {
                id:            x.id.into(),
                name:          x.name,
                structure_ids: structure_ids,
            }
        })
        .map_err(|e| Error::FetchGroup(e, group_uuid))
}

