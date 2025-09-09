use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::{CategoryId, CharacterId, ConstellationId, GroupId, RegionId, SystemId, TypeId};

use crate::{Error, Result, Security, StructureListFilter, StructureRig, StructureUuid};
use utoipa::ToSchema;

pub async fn list(
    pool:         &PgPool,
    character_id: CharacterId,
    filter:       StructureListFilter,
) -> Result<Vec<StructureDatabase>> {
    let entries = sqlx::query!(r#"
            SELECT
                structure.id,
                structure.structure_id,
                structure.name AS "structure_name",
                structure.security AS "security_group!: Security",
                structure.services,
                structure.rigs,
                system.*,
                item.*
            FROM structure
            JOIN system ON system.system_id = structure.system_id
            JOIN item ON item.type_id = structure.type_id
            WHERE
                NOT (LOWER(structure.name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (structure.system_id = $3) IS FALSE AND
                NOT (structure.type_id = $4) IS FALSE AND
                NOT ($5 = ANY(services)) IS FALSE AND
                (owner = $1 OR owner = 0)
                ORDER BY structure.name
        "#,
            *character_id,
            filter.name,
            filter.system_id,
            filter.structure_type_id,
            filter.service_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::ListStructureIds(e, character_id, filter))?;

    // TODO: refactor so that rigs are included
    let mut structures = Vec::new();
    for entry in entries {
        let mut rigs = Vec::new();

        for rig in entry.rigs {
            rigs.push(
                crate::rig::fetch(
                    pool,
                    TypeId::from(rig)
                )
                .await?
            );
        }

        let mut services = Vec::new();
        for service in entry.services {
            let entry = sqlx::query!("
                    SELECT *
                    FROM item
                    WHERE type_id = $1
                ",
                    service,
                )
                .fetch_one(pool)
                .await
                .unwrap();

            let service= StructureDatabaseType {
                type_id:       entry.type_id.into(),
                category_id:   entry.category_id.into(),
                group_id:      entry.group_id.into(),
                meta_group_id: entry.meta_group_id.map(Into::into),
                base_price:    entry.base_price,
                volume:        entry.volume,
                name:          entry.name,
                repackaged:    entry.repackaged,
            };
            services.push(service);
        }

        let structure = StructureDatabase {
            id:                     entry.id.into(),
            name:                   entry.structure_name,
            structure_id:           entry.structure_id,
            system: StructureDatabaseSystem {
                constellation_id:   entry.constellation_id.into(),
                constellation_name: entry.constellation_name,
                region_id:          entry.region_id.into(),
                region_name:        entry.region_name,
                system_id:          entry.system_id.into(),
                system_name:        entry.system_name,
                security:           entry.security,
                security_group:     entry.security_group,
            },
            structure_type: StructureDatabaseType {
                type_id:            entry.type_id.into(),
                category_id:        entry.category_id.into(),
                group_id:           entry.group_id.into(),
                meta_group_id:      entry.meta_group_id.map(Into::into),
                base_price:         entry.base_price,
                volume:             entry.volume,
                name:               entry.name,
                repackaged:         entry.repackaged,
            },
            rigs:                   rigs,
            services:               services,
        };

        structures.push(structure);
    }

    Ok(structures)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct StructureDatabase {
    /// Internal id of the structure
    pub id:                StructureUuid,
    /// EVE Id of the structure
    pub structure_id:      i64,
    /// Name of the structure
    pub name:              String,
    /// Location of the structure
    pub system:            StructureDatabaseSystem,
    /// Type information
    pub structure_type:    StructureDatabaseType,
    /// List of all rigs that are in the structure
    pub rigs:              Vec<StructureRig>,
    /// Id of the structure in-game
    pub services:          Vec<StructureDatabaseType>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct StructureDatabaseSystem {
    pub region_id:          RegionId,
    pub region_name:        String,

    pub constellation_id:   ConstellationId,
    pub constellation_name: String,

    pub system_id:          SystemId,
    pub system_name:        String,

    pub security:           f32,
    pub security_group:     Security,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct StructureDatabaseType {
    pub type_id:       TypeId,
    pub category_id:   CategoryId,
    pub group_id:      GroupId,
    pub meta_group_id: Option<GroupId>,
    pub base_price:    Option<f32>,
    pub volume:        f32,
    pub name:          String,
    pub repackaged:    Option<i32>,
}
