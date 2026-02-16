use sqlx::PgPool;
use std::str::FromStr;
use uuid::{NoContext, Timestamp, Uuid};

use crate::Mapping;

pub async fn migrate_project_group(
    postgres_source:      &PgPool,
    postgres_destination: &PgPool,
    mappings:             &mut Mapping,
) -> Result<(), Box<dyn std::error::Error>> {
    dbg!("Start - project group");

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
        } else if project_group.id == Uuid::from_str("adbd22d0-0981-47ec-98d0-244b6d4ecd10").unwrap() {
            // FIS project group
            continue;
        } else if project_group.id == Uuid::from_str("65a6a5eb-1195-4f9d-a173-5e064afb845b").unwrap() {
            // Eve Goal project group
            continue;
        }

        let project_group_id = if let Some(x) = mappings.get(&project_group.id) {
            x.clone()
        } else {
            let timestamp = Timestamp::from_unix(NoContext, project_group.created_at.timestamp() as u64, 0);
            let project_group_id = Uuid::new_v7(timestamp);
            mappings.insert(project_group.id, project_group_id);
            project_group_id
        };

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
                ON CONFLICT (id)
                DO UPDATE SET
                    name        = EXCLUDED.name,
                    owner       = EXCLUDED.owner,
                    updated_at  = EXCLUDED.updated_at
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
                DELETE FROM project_group_default_blacklist
                WHERE project_group_id = $1
            ",
                project_group_id,
            )
            .execute(&mut *transaction)
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
                DELETE FROM project_group_default_market
                WHERE project_group_id = $1
            ",
                project_group_id,
            )
            .execute(&mut *transaction)
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
                    .map(|x| mappings.get(&x).unwrap().clone())
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
                DELETE FROM project_group_member
                WHERE project_group_id = $1
            ",
                project_group_id,
            )
            .execute(&mut *transaction)
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
                    DELETE FROM project_group_default_job_splitting_general
                    WHERE project_group_id = $1
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
                    DELETE FROM project_group_default_job_splitting_run
                    WHERE project_group_id = $1
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
                        -- Capital Armor Plates
                        ($1, 21017, 40),
                        -- Capital Capacitor Battery
                        ($1, 21019, 40),
                        -- Capital Cargo Bay
                        ($1, 21027, 40),
                        -- Capital Clone Vat Bay
                        ($1, 24547, 40),
                        -- Capital Computer System
                        ($1, 21035, 40),
                        -- Capital Construction Parts
                        ($1, 21037, 40),
                        -- Capital Corporate Hangar Bay
                        ($1, 24560, 40),
                        -- Capital Doomsday Weapon Mount
                        ($1, 24556, 40),
                        -- Capital Drone Bay
                        ($1, 21029, 40),
                        -- Capital Jump Bridge Array
                        ($1, 24545, 40),
                        -- Capital Jump Drive
                        ($1, 21025, 40),
                        -- Capital Launcher Hardpoint
                        ($1, 21041, 40),
                        -- Capital Power Generator
                        ($1, 21021, 40),
                        -- Capital Propulsion Engine
                        ($1, 21009, 40),
                        -- Capital Sensor Cluster
                        ($1, 21013, 40),
                        -- Capital Shield Emitter
                        ($1, 21023, 40),
                        -- Capital Ship Maintenance Bay
                        ($1, 24558, 40),
                        -- Capital Siege Array
                        ($1, 21039, 40),
                        -- Capital Turret Hardpoint
                        ($1, 21011, 40),
                        -- Gravimetric-FTL Interlink Communicator
                        ($1, 57475, 40),
                        -- Ladar-FTL Interlink Communicator
                        ($1, 57477, 40),
                        -- Magnetometric-FTL Interlink Communicator
                        ($1, 57476, 40),
                        -- Radar-FTL Interlink Communicator
                        ($1, 57474, 40),
                        -- Life Support Backup Unit
                        ($1, 57486, 200),
                        -- Auto-Integrity Preservation Seal
                        ($1, 57478, 200),
                        -- Core Temperature Regulator
                        ($1, 57479, 40),
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

    Ok(())
}
