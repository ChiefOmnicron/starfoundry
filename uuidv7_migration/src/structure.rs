use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::Mapping;
use std::str::FromStr;

pub async fn migrate_structure(
    postgres_source: &PgPool,
    postgres_destination: &PgPool,
) -> Result<Mapping, Box<dyn std::error::Error>> {
    struct StructurePosition {
        x: f32,
        y: f32,
        z: f32,
    }

    let mut coordinates = HashMap::new();
    // 4-P4FE - Advanced Components
    coordinates.insert(1047505499925i64, StructurePosition { x: -170027240000f32, y: -4523340000f32, z: -557097600000f32 });
    // 4-P4FE - Capital & Large Ships
    coordinates.insert(1049960174130i64, StructurePosition { x: -170027750000f32, y: -4523341000f32, z: -557103840000f32 });
    // 4-P4FE - Capital Components
    coordinates.insert(1049960199427i64, StructurePosition { x: -170027160000f32, y: -4523341300f32, z: -557096500000f32 });
    // 4-P4FE - Reactions & Reprocessing
    coordinates.insert(1047460909336i64, StructurePosition { x: -170027660000f32, y: -4523335700f32, z: -557102700000f32 });
    // ABE-M2 - Caution Reactor -Rami
    coordinates.insert(1049992497867i64, StructurePosition { x: 1272967600000f32, y: 1113184300000f32, z: 1392066800000f32 });
    // ABE-M2 - Capital Components
    coordinates.insert(1049960427135i64, StructurePosition { x: 1272963000000f32, y: 1113184300000f32, z: 1392062000000f32 });
    // ABE-M2 - Advanced Large and Medium
    coordinates.insert(1050623036026i64, StructurePosition { x: 1272962100000f32, y: 1113184300000f32, z: 1392060800000f32 });
    // ABE-M2 - Basic Large and Medium
    coordinates.insert(1050623008788i64, StructurePosition { x: 1272960900000f32, y: 1113184300000f32, z: 1392059600000f32 });
    // ABE-M2 - Advanced Components + Av Sm Ship
    coordinates.insert(1049960433055i64, StructurePosition { x: 1272959000000f32, y: 1113184300000f32, z: 1392057500000f32 });
    // ABE-M2 - Equipment, Drones, Structure
    coordinates.insert(1050622991947i64, StructurePosition { x: 1272960000000f32, y: 1113184300000f32, z: 1392058600000f32 });
    // ABE-M2 - RCI Made Me too
    coordinates.insert(1049960387273i64, StructurePosition { x: 1272965600000f32, y: 1113184300000f32, z: 1392064700000f32 });
    // C-J6MT - 1st Taj Mahgoon
    coordinates.insert(1049588174021i64, StructurePosition { x: -3377780000000f32, y: -155983800000f32, z: -3188423100000f32 });
    // RH0-EG - Big Atron Shipyards Mk2
    coordinates.insert(1050206773809i64, StructurePosition { x: 465144200000f32, y: -29717176000f32, z: 1673478400000f32 });
    // UALX-3 - Mothership Bellicose
    coordinates.insert(1046664001931i64, StructurePosition { x: 1643733600000f32, y: -76999926000f32, z: -262765690000f32 });
    // Jita
    coordinates.insert(60003760, StructurePosition { x: -107303362560f32, y: -18744975360f32, z: 436489052160f32 });
    // Amarr
    coordinates.insert(60008494, StructurePosition { x: -518583951360f32, y: 30256619520f32, z: 1042895708160f32 });

    dbg!("Start - structure");
    let mut mappings = HashMap::new();

    let structures = sqlx::query!(r#"
            SELECT
                id,
                structure_id,
                system_id,
                type_id,
                rigs,
                services,
                name,
                owner,
                created_at,
                updated_at
            FROM structure
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut transaction = postgres_destination.begin().await?;
    for structure in structures {
        let coordinates = if let Some(x) = coordinates.get(&structure.structure_id) {
            x
        } else {
            continue;
        };

        let structure_id = if structure.id == Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap() {
            Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap()
        } else if  structure.id == Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap() {
            Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap()
        } else {
            Uuid::now_v7()
        };
        mappings.insert(structure.id, structure_id);

        let services = if structure.id == Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap() {
            vec![35892]
        } else if structure.id == Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap() {
            vec![35892]
        } else {
            structure.services
        };

        let type_id = if structure.id == Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap() {
            52678
        } else if structure.id == Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap() {
            46767
        } else {
            structure.type_id
        };

        sqlx::query!("
                INSERT INTO structure
                (
                    id,
                    structure_id,
                    system_id,
                    type_id,
                    rigs,
                    services,
                    name,
                    owner,
                    x,
                    y,
                    z,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ",
                structure_id,
                structure.structure_id,
                structure.system_id,
                type_id,
                &structure.rigs,
                &services,
                structure.name,
                structure.owner,
                coordinates.x,
                coordinates.y,
                coordinates.z,
                structure.created_at,
                structure.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;
    dbg!("Done - structure");
    Ok(mappings)
}
