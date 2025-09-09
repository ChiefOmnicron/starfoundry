use sqlx::PgPool;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::str::FromStr;
use uuid::Uuid;

use crate::{Error, Result, Security, Structure, StructureDatabase, StructureDatabaseSystem, StructureDatabaseType, StructureType, StructureUuid};

#[deprecated]
pub async fn fetch(
    pool:           &PgPool,
    character_id:   CharacterId,
    structure_uuid: StructureUuid,
) -> Result<Option<StructureDatabase>> {
    //if let Some(x) = npc_station(structure_uuid) {
    //    return Ok(Some(x));
    //}

    let entry = sqlx::query!(r#"
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
            WHERE owner = $1
            AND id = $2
        "#,
            *character_id,
            *structure_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchStructure(e, structure_uuid))?;

    if let Some(entry) = entry {
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
                volume:             entry.volume,
                name:               entry.name,
                repackaged:         entry.repackaged,
            },
            rigs:                   rigs,
            services:               services,
        };
        Ok(Some(structure))
    } else {
        Err(Error::StructureNotFound(structure_uuid))
    }
}

fn npc_station(
    structure_uuid: StructureUuid,
) -> Option<Structure> {
    if structure_uuid == StructureUuid(Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap_or_default()) {
        Some(Structure {
            id: StructureUuid(Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap_or_default()),
            name: "Jita 4-4".into(),
            rigs: Vec::new(),
            security: Security::Highsec,
            services: Vec::new(),
            structure_id: 60003760,
            structure_type: StructureType::Invalid,
            system_id: 30000142.into(),
        })
    } else if structure_uuid == StructureUuid(Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap_or_default()) {
        Some(Structure {
            id: StructureUuid(Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap_or_default()),
            name: "Amarr".into(),
            rigs: Vec::new(),
            security: Security::Highsec,
            services: Vec::new(),
            structure_id: 60008494,
            structure_type: StructureType::Invalid,
            system_id: 30002187.into(),
        })
    } else {
        None
    }
}

// FIXME:
// required for checking if there are enough ressources for job assignments
pub async fn danger_no_permission_fetch(
    pool:           &PgPool,
    structure_uuid: StructureUuid,
) -> Result<Option<Structure>> {
    if let Some(x) = npc_station(structure_uuid) {
        return Ok(Some(x));
    }

    let result = sqlx::query!(r#"
            SELECT
                id,
                name,
                structure.system_id,
                system_name,
                structure_id,
                structure.security  AS "security!: Security",
                type_id,
                rigs,
                services
            FROM structure
            JOIN system ON system.system_id = structure.system_id
            WHERE id = $1
        "#,
            *structure_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchStructure(e, structure_uuid))?;

    if let Some(x) = result {
        let mut rigs = Vec::new();

        for rig in x.rigs {
            rigs.push(
                crate::rig::fetch(
                    pool,
                    TypeId::from(rig)
                )
                .await?
            );
        }

        let structure = Structure {
            id:             x.id.into(),
            name:           x.name,
            system_id:      x.system_id.into(),
            structure_id:   x.structure_id,
            security:       x.security,
            structure_type: StructureType::from(x.type_id),
            services:       x.services
                                .into_iter()
                                .map(Into::into)
                                .collect::<Vec<_>>(),
            rigs:           rigs,
        };
        Ok(Some(structure))
    } else {
        Err(Error::StructureNotFound(structure_uuid))
    }
}
