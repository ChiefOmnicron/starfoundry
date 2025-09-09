use sqlx::PgPool;
use starfoundry_lib_projects::{group_structures, ProjectStructureGroup};
use starfoundry_lib_structures::StructureGroupUuid;
use starfoundry_lib_types::CharacterId;

pub async fn structure_groups(
    pool: &PgPool,
    group_uuids: Vec<StructureGroupUuid>,
) -> Result<Vec<ProjectStructureGroup>, Box<dyn std::error::Error>> {
    let mut groups = Vec::new();

    for group_id in group_uuids {
        let structure_groups = group_structures(
                &pool,
                CharacterId(2117441999),
                group_id,
            ).await?;

        groups.extend(structure_groups);
    }

    Ok(groups)
}
