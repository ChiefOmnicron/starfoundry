use sqlx::PgPool;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::{NoContext, Timestamp, Uuid};

use crate::Mapping;

pub async fn migrate_project_group(
    postgres_source:      &PgPool,
    postgres_destination: &PgPool,
    structure_mapping:    &Mapping,
) -> Result<Mapping, Box<dyn std::error::Error>> {
    dbg!("Start - project group");
    let mut mappings = HashMap::new();

    let project_groups = sqlx::query!(r#"
            SELECT
                id,
                name,
                owner,
                description,
                created_at,
                updated_at
            FROM project_group
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut transaction = postgres_destination.begin().await?;
    for project_group in project_groups {
        if project_group.id == Uuid::default() {
            continue;
        } else if project_group.id == Uuid::from_str("dda037ce-9f5c-4a0f-8767-db5b38575b91").unwrap() {
            continue;
        } else if project_group.id == Uuid::from_str("5d0033ac-3ac9-4890-a00a-634dc2e69bf8").unwrap() {
            // AA project group
            continue;
        }

        let timestamp = Timestamp::from_unix(NoContext, project_group.created_at.timestamp() as u64, 0);
        let project_group_id = Uuid::new_v7(timestamp);
        mappings.insert(project_group.id, project_group_id);

        sqlx::query!("
                INSERT INTO project_group
                (
                    id,
                    name,
                    owner,
                    description,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
                project_group_id,
                project_group.name,
                project_group.owner,
                project_group.description,
                project_group.created_at,
                project_group.updated_at,
            )
            .execute(&mut *transaction)
            .await?;

        let blacklist = sqlx::query!("
                SELECT
                    project_group_id,
                    type_id,
                    created_at,
                    updated_at
                FROM project_group_default_blacklist
                WHERE project_group_id = $1
            ",
                project_group.id
            )
            .fetch_all(postgres_source)
            .await?;
        sqlx::query!("
                INSERT INTO project_group_default_blacklist
                (
                    project_group_id,
                    type_id,
                    created_at,
                    updated_at
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::TIMESTAMPTZ[],
                    $4::TIMESTAMPTZ[]
                )
            ",
                project_group_id,
                &blacklist.iter().map(|x| x.type_id).collect::<Vec<_>>(),
                &blacklist.iter().map(|x| x.created_at).collect::<Vec<_>>(),
                &blacklist.iter().map(|x| x.updated_at).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await?;

        let market = sqlx::query!("
                SELECT
                    project_group_id,
                    structure_id,
                    created_at,
                    updated_at
                FROM project_group_default_market
                WHERE project_group_id = $1
            ",
                project_group.id
            )
            .fetch_all(postgres_source)
            .await?;
        sqlx::query!("
                INSERT INTO project_group_default_market
                (
                    project_group_id,
                    structure_id,
                    created_at,
                    updated_at
                )
                SELECT $1, * FROM UNNEST(
                    $2::UUID[],
                    $3::TIMESTAMPTZ[],
                    $4::TIMESTAMPTZ[]
                )
            ",
                project_group_id,
                &market
                    .iter()
                    .map(|x| x.structure_id)
                    .map(|x| structure_mapping.get(&x).unwrap().clone())
                    .collect::<Vec<_>>(),
                &market.iter().map(|x| x.created_at).collect::<Vec<_>>(),
                &market.iter().map(|x| x.updated_at).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await?;

        let member = sqlx::query!("
                SELECT
                    group_id,
                    character_id,
                    accepted,
                    created_at,
                    updated_at
                FROM project_group_member
                WHERE group_id = $1
            ",
                project_group.id
            )
            .fetch_all(postgres_source)
            .await?;
        sqlx::query!("
                INSERT INTO project_group_member
                (
                    project_group_id,
                    character_id,
                    accepted,
                    permission,
                    created_at,
                    updated_at
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::BOOLEAN[],
                    $4::INTEGER[],
                    $5::TIMESTAMPTZ[],
                    $6::TIMESTAMPTZ[]
                )
            ",
                project_group_id,
                &member
                    .iter()
                    .map(|x| x.character_id)
                    .collect::<Vec<_>>(),
                &member
                    .iter()
                    .map(|x| x.accepted)
                    .collect::<Vec<_>>(),
                &member
                    .iter()
                    .map(|x| {
                        return if project_group.owner == x.character_id {
                            1
                        } else {
                            2
                        }
                    })
                    .collect::<Vec<_>>(),
                &member.iter().map(|x| x.created_at).collect::<Vec<_>>(),
                &member.iter().map(|x| x.updated_at).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await?;

        if project_group.id == Uuid::from_str("ec3a9410-ee92-4435-925d-b81a7a987891").unwrap() {
            sqlx::query!("
                    INSERT INTO project_group_default_blueprint_overwrite
                    (
                        project_group_id,
                        type_id,
                        material_efficiency
                    )
                    VALUES
                    -- Providence Blueprint
                    ($1, 20184, 8),
                    -- Charon Blueprint
                    ($1, 20186, 8),
                    -- Obelisk Blueprint
                    ($1, 20188, 8),
                    -- Fenrir Blueprint
                    ($1, 20190, 8),
                    -- Capital Ultratidal Entropic Unit Blueprint
                    ($1, 53032, 0),
                    -- Capital Radiation Conversion Unit Blueprint
                    ($1, 53033, 0),
                    -- Capital Absorption Thruster Array Blueprint
                    ($1, 53034, 0)
                ",
                    project_group_id,
                )
                .execute(&mut *transaction)
                .await?;
            sqlx::query!("
                    INSERT INTO project_group_default_job_splitting_general
                    (
                        project_group_id
                    )
                    VALUES ($1)
                ",
                    project_group_id,
                )
                .execute(&mut *transaction)
                .await?;
            sqlx::query!("
                    INSERT INTO project_group_default_job_splitting_run
                    (
                        project_group_id,
                        type_id,
                        max_runs
                    )
                    VALUES
                        -- Capital Armor Plates Blueprint
                        ($1, 21018, 40),
                        -- Capital Capacitor Battery Blueprint
                        ($1, 21020, 40),
                        -- Capital Cargo Bay Blueprint
                        ($1, 21028, 40),
                        -- Capital Clone Vat Bay Blueprint
                        ($1, 24548, 40),
                        -- Capital Computer System Blueprint
                        ($1, 21036, 40),
                        -- Capital Construction Parts Blueprint
                        ($1, 21038, 40),
                        -- Capital Corporate Hangar Bay Blueprint
                        ($1, 24561, 40),
                        -- Capital Doomsday Weapon Mount Blueprint
                        ($1, 24557, 40),
                        -- Capital Drone Bay Blueprint
                        ($1, 21030, 40),
                        -- Capital Jump Bridge Array Blueprint
                        ($1, 24546, 40),
                        -- Capital Jump Drive Blueprint
                        ($1, 21026, 40),
                        -- Capital Launcher Hardpoint Blueprint
                        ($1, 21042, 40),
                        -- Capital Power Generator Blueprint
                        ($1, 21022, 40),
                        -- Capital Propulsion Engine Blueprint
                        ($1, 21010, 40),
                        -- Capital Sensor Cluster Blueprint
                        ($1, 21014, 40),
                        -- Capital Shield Emitter Blueprint
                        ($1, 21024, 40),
                        -- Capital Ship Maintenance Bay Blueprint
                        ($1, 24559, 40),
                        -- Capital Siege Array Blueprint
                        ($1, 21040, 40),
                        -- Capital Turret Hardpoint Blueprint
                        ($1, 21012, 40),
                        -- Gravimetric-FTL Interlink Communicator Blueprint
                        ($1, 57512, 40),
                        -- Ladar-FTL Interlink Communicator Blueprint
                        ($1, 57514, 40),
                        -- Magnetometric-FTL Interlink Communicator Blueprint
                        ($1, 57513, 40),
                        -- Radar-FTL Interlink Communicator Blueprint
                        ($1, 57511, 40),
                        -- Life Support Backup Unit Blueprint
                        ($1, 57523, 200),
                        -- Auto-Integrity Preservation Seal Blueprint
                        ($1, 57515, 200),
                        -- Core Temperature Regulator Blueprint
                        ($1, 57516, 40),
                        -- G-O Trigger Neurolink Conduit Blueprint
                        ($1, 57510, 200),
                        -- R-O Trigger Neurolink Conduit Blueprint
                        ($1, 57508, 200),
                        -- S-R Trigger Neurolink Conduit Blueprint
                        ($1, 57509, 200),
                        -- U-C Trigger Neurolink Conduit Blueprint
                        ($1, 57507, 200)
                ",
                    project_group_id,
                )
                .execute(&mut *transaction)
                .await?;
        }
    }
    transaction.commit().await?;
    dbg!("Done - project group");

    Ok(mappings)
}
