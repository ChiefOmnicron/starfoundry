use sqlx::PgPool;
use starfoundry_lib_structures::{StructureGroupUuid, StructureService, StructureUuid};
use starfoundry_lib_types::CharacterId;

use crate::{Error, ProjectStructureGroup, Result, StructureMapping};

// TODO: validate if this needs to be moved
pub async fn group_structures(
    pool:         &PgPool,
    character_id: CharacterId,
    group_id:     StructureGroupUuid,
) -> Result<Vec<ProjectStructureGroup>> {
    let mut groups = Vec::new();

    // first try to find a regular structure group
    if let Some(x) = structure_group(pool, character_id, group_id).await? {
        groups.push(x);
    } else {
        let structures = structure_dynamic_groups(pool, character_id, group_id).await?;
        groups.extend(structures);
    }

    Ok(groups)
}

/// Gets a structure group and parses its strucutres
async fn structure_group(
    pool:         &PgPool,
    character_id: CharacterId,
    group_id:     StructureGroupUuid,
) -> Result<Option<ProjectStructureGroup>> {
    let structure_uuids = sqlx::query!("
            SELECT structure_id
            FROM structure_group_structure
            WHERE structure_group_id = $1
        ",
            *group_id,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::FetchStructureGroup)?
        .into_iter()
        .map(|x| x.structure_id)
        .map(Into::into)
        .collect::<Vec<_>>();

    let mut group = ProjectStructureGroup::default();
    group.id = *group_id;

    for structure_uuid in structure_uuids {
        let structure = StructureService::new(structure_uuid)
            .danger_no_permission_fetch(pool)
            .await?
            .ok_or_else(|| Error::StructureNotFound(structure_uuid))?;

        group
            .mapping
            .push(StructureMapping {
                structure_uuid: structure_uuid,
                category_group: structure.category_groups(),
            });

        group
            .system_ids
            .push(structure.system_id);

        group
            .structures
            .push(structure);
    }

    return Ok(Some(group));
}

async fn structure_dynamic_groups(
    pool:         &PgPool,
    character_id: CharacterId,
    group_id:     StructureGroupUuid,
) -> Result<Vec<ProjectStructureGroup>> {
    let mut groups = Vec::new();

    let group_ids = sqlx::query!("
            SELECT group_ids
            FROM structure_dynamic_group
            WHERE id = $1
        ",
            *group_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| x.group_ids)
        .map_err(Error::FetchDynamicStructureGroup)?;

    for group_id in group_ids {
        // silently fail if a strucutre cannot be resolved
        if let Ok(Some(x)) = structure_group(pool, character_id, group_id.into()).await {
            groups.push(x);
        }
    }

    Ok(groups)
}
