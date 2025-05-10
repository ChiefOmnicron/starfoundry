use sqlx::PgPool;
use starfoundry_libs_types::{CharacterId, TypeId};
use std::str::FromStr;
use uuid::Uuid;

use crate::{Error, Result, Structure, StructureType, StructureUuid, Security};

pub async fn fetch(
    pool:           &PgPool,
    character_id:   CharacterId,
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
            WHERE owner = $1
            AND id = $2
        "#,
            *character_id,
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
