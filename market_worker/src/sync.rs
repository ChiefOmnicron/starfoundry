use chrono::{NaiveDateTime, Timelike, Utc};
use sqlx::PgPool;
use starfoundry_lib_types::{CharacterId, RegionId, StructureId};
use starfoundry_lib_worker::Task;
use std::collections::HashMap;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::WorkerMarketTask;

/// Ensures that all necessary tasks are in the queue and new structures
/// are added into the rotation
pub async fn sync_task(
    pool:   &PgPool,
    task:   &mut Task<WorkerMetric, WorkerMarketTask>,
) -> Result<()> {
    match sync_player_stations(
        pool,
    ).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.append_log(format!("added {new_entries} player markets"))
            }
        },
        Err(e) => task.append_error(e.to_string()),
    };

    match sync_npc_stations(
        pool,
    ).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.append_log(format!("added {new_entries} npc markets"))
            }
        },
        Err(e) => task.append_error(e.to_string()),
    };

    match sync_npc_stations_authed(
        pool,
    ).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.append_log(format!("added {new_entries} npc markets"))
            }
        },
        Err(e) => task.append_error(e.to_string()),
    };

    match sync_public_contract(
        pool,
    ).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.append_log(format!("added {new_entries} public contracts"))
            }
        },
        Err(e) => task.append_error(e.to_string()),
    };

    match sync_misc_tasks(
        pool,
    ).await {
        Ok(_) => {},
        Err(e) => task.append_error(e.to_string()),
    };

    Ok(())
}

/// Ensures that all necessary tasks are in the queue and new structures
/// are added into the rotation
pub async fn sync(
    pool:   &PgPool,
) -> Result<()> {
    sync_player_stations(
        pool,
    ).await?;

    /*sync_private_orders(
        pool,
    ).await?;*/

    sync_npc_stations(
        pool,
    ).await?;

    sync_npc_stations_authed(
        pool,
    ).await?;

    sync_public_contract(
        pool,
    ).await?;

    sync_misc_tasks(
        pool,
    ).await?;

    Ok(())
}

async fn sync_misc_tasks(
    pool: &PgPool,
) -> Result<()> {
    for task in vec![
        WorkerMarketTask::Sync,
        WorkerMarketTask::Cleanup,
        WorkerMarketTask::Prices,
    ] {
        let task: String = task.into();

        let task_db = sqlx::query!("
                SELECT 1 as exists
                FROM worker_queue
                WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
                AND task = $1
            ",
                task,
            )
            .fetch_optional(pool)
            .await
            .map_err(Error::SyncError)?;

        if task_db.is_none() {
            sqlx::query!("
                INSERT INTO worker_queue (task)
                SELECT $1
            ",
                task,
            )
            .execute(pool)
            .await
            .map_err(Error::SyncError)?;
        }
    }

    Ok(())
}

async fn sync_npc_stations(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerMarketTask::LatestNpc.into();

    // (structureId, regionId)
    let structures = vec![
        // Jita
        (60003760, 10000002),
        // Amarr
        (60008494, 10000043),
    ];

    let market_stations = sqlx::query!("
            SELECT
                (additional_data ->> 'structure_id')::BIGINT AS structure_id,
                (additional_data ->> 'region_id')::INTEGER AS region_id
            FROM worker_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = $1
        ",
            &task_name,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?;

    // ensure that non authed structures are in the queue
    let mut new_entries = Vec::new();
    for (structure_id, region_id) in structures {
        if let None = market_stations
            .iter()
            .find(|x| {
                x.region_id == Some(region_id) &&
                x.structure_id == Some(structure_id)
            }) {

            let additional_data = serde_json::json!({
                "structure_id": structure_id,
                "region_id": region_id,
            });
            new_entries.push(additional_data);
        }
    }

    tracing::info!("Added {} new npc market jobs", new_entries.len());
    sqlx::query!("
            INSERT INTO worker_queue (task, additional_data)
            SELECT $1, * FROM UNNEST(
                $2::JSONB[]
            )
        ",
            &task_name,
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::SyncError)
}

async fn sync_npc_stations_authed(
    pool: &PgPool,
) -> Result<usize> {
    // authed requests
    let mut registered_structures = HashMap::new();
    sqlx::query!("
            SELECT
                main_character,
                character_id,
                structure_id,
                region_id,
                source
            FROM structure
            WHERE structure_id < 1000000000000
        ")
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?
        .into_iter()
        .map(|x| TimeSlottedMarketEntry {
            main_character: x.main_character.into(),
            character_id:   x.character_id.into(),
            structure_id:   x.structure_id.into(),
            region_id:      x.region_id.into(),
            source:         x.source,
        })
        .for_each(|x| {
            registered_structures
                .entry((x.main_character, x.structure_id))
                .and_modify(|y: &mut Vec<TimeSlottedMarketEntry>| y.push(x.clone()))
                .or_insert(vec![x]);
        });

    time_slotted_market(
        pool,
        WorkerMarketTask::LatestNpcAuthed,
        registered_structures,
    ).await
}

async fn sync_player_stations(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerMarketTask::LatestPlayer.into();

    let mut registered_structures = HashMap::new();
    sqlx::query!("
            SELECT
                main_character,
                character_id,
                structure_id,
                region_id,
                source
            FROM structure
            WHERE structure_id > 1000000000000
        ")
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?
        .into_iter()
        .map(|x| TimeSlottedMarketEntry {
            main_character: x.main_character.into(),
            character_id:   x.character_id.into(),
            structure_id:   x.structure_id.into(),
            region_id:      x.region_id.into(),
            source:         x.source,
        })
        .for_each(|x| {
            registered_structures
                .entry((x.main_character, x.structure_id))
                .and_modify(|y: &mut Vec<TimeSlottedMarketEntry>| y.push(x.clone()))
                .or_insert(vec![x]);
        });

    let market_stations = sqlx::query!(r#"
            SELECT
                (additional_data ->> 'structure_id')::BIGINT AS "structure_id!",
                (additional_data ->> 'character_id')::INTEGER AS "character_id!",
                (additional_data ->> 'source')::VARCHAR AS source
            FROM worker_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = $1
        "#,
            &task_name,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?;

    // if there is at least one entry, skip it
    if !market_stations.is_empty() {
        return Ok(0usize);
    }

    time_slotted_market(
        pool,
        WorkerMarketTask::LatestPlayer,
        registered_structures,
    ).await
}

async fn sync_public_contract(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerMarketTask::PublicContracts.into();
    let mut new_entries = Vec::new();

    // FIXME:_ configurable
    for region_id in vec![10000002, 10000043] {
        let public_contract = sqlx::query!("
                SELECT
                    (additional_data ->> 'region_id')::INTEGER AS region_id
                FROM worker_queue
                WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
                AND task = $1
            ",
                &task_name,
            )
            .fetch_optional(pool)
            .await
            .map_err(Error::SyncError)?;

        if public_contract.is_none() {
            new_entries.push(serde_json::json!({
                "region_id": region_id,
            }))
        }
    }

    tracing::info!("Added {} new player market jobs", new_entries.len());
    sqlx::query!("
            INSERT INTO worker_queue (task, additional_data)
            SELECT $1, * FROM UNNEST(
                $2::JSONB[]
            )
        ",
            &task_name,
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::SyncError)
}

async fn sync_private_orders(
    pool: &PgPool,
    host: String,
) -> Result<usize> {
    let mut total_added = 0;

    // FIXME: only tmp
    for character_id in vec![2117441999] {
        let mut new_entries = Vec::new();
        let task_name: String = WorkerMarketTask::CharacterOrders.into();
        let public_contract = sqlx::query!("
                SELECT
                    (additional_data ->> 'character_id')::INTEGER AS character_id
                FROM worker_queue
                WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
                AND task = $1
            ",
                &task_name,
            )
            .fetch_optional(pool)
            .await
            .map_err(Error::SyncError)?;

        if public_contract.is_none() {
            new_entries.push(serde_json::json!({
                "character_id": character_id,
                "source": host.clone(),
            }))
        }

        tracing::info!("Added {} new character orders", new_entries.len());
        sqlx::query!("
                INSERT INTO worker_queue (task, additional_data)
                SELECT $1, * FROM UNNEST(
                    $2::JSONB[]
                )
            ",
                &task_name,
                &new_entries
            )
            .execute(pool)
            .await
            .map(|_| new_entries.len())
            .map_err(Error::SyncError)?;
        total_added += new_entries.len();
    }

    // FIXME: only tmp
    for corporation_id in vec![98024275] {
        let mut new_entries = Vec::new();
        let task_name: String = WorkerMarketTask::CorporationOrders.into();
        let public_contract = sqlx::query!("
                SELECT
                    (additional_data ->> 'corporation_id')::INTEGER AS corporation_id
                FROM worker_queue
                WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
                AND task = $1
            ",
                &task_name,
            )
            .fetch_optional(pool)
            .await
            .map_err(Error::SyncError)?;

        if public_contract.is_none() {
            new_entries.push(serde_json::json!({
                "corporation_id": corporation_id,
                "character_id": 2117441999,
                "source": host.clone(),
            }))
        }

        tracing::info!("Added {} new corporation orders", new_entries.len());
        sqlx::query!("
                INSERT INTO worker_queue (task, additional_data)
                SELECT $1, * FROM UNNEST(
                    $2::JSONB[]
                )
            ",
                &task_name,
                &new_entries
            )
            .execute(pool)
            .await
            .map(|_| new_entries.len())
            .map_err(Error::SyncError)?;
        total_added += new_entries.len();
    }

    Ok(total_added)
}

/// Market entries that are time slotted
async fn time_slotted_market(
    pool:      &PgPool,
    task_name: WorkerMarketTask,
    entries:   HashMap<(CharacterId, StructureId), Vec<TimeSlottedMarketEntry>>,
) -> Result<usize> {
    let task_name: String = task_name.into();

    #[derive(Clone, Debug)]
    struct TaskInformation {
        process_after:   NaiveDateTime,
        additional_data: serde_json::Value,
    }

    let mut new_entries = Vec::new();
    for (_, structures) in entries {
        let mut current_time_slot = Utc::now().naive_utc();
        // 300 = 5 minutes in seconds
        // at max 10 slots are available
        let time_slot_diff = 300 / std::cmp::min(10, structures.len());

        'outer: loop {
            for structure in structures.iter() {
                let date = current_time_slot;
                if date.time().hour() == 11 && date.time().minute() < 30 {
                    // during downtime new entries should be generated
                    if new_entries.is_empty() {
                        date
                    } else {
                        break 'outer;
                    }
                } else {
                    date
                };

                new_entries.push(TaskInformation {
                    process_after: current_time_slot,
                    additional_data: serde_json::json!({
                        "character_id": structure.character_id,
                        "region_id": structure.region_id,
                        "structure_id": structure.structure_id,
                        "source": structure.source,
                    }),
                });
                current_time_slot += Duration::from_secs(time_slot_diff as u64);
            }
        }
    }

    tracing::error!("Added {} new player market jobs", new_entries.len());
    sqlx::query!("
            INSERT INTO worker_queue (task, process_after, additional_data)
            SELECT $1, * FROM UNNEST(
                $2::TIMESTAMP[],
                $3::JSONB[]
            )
        ",
            &task_name,
            &new_entries.iter().map(|x| x.process_after).collect::<Vec<_>>(),
            &new_entries.iter().map(|x| x.additional_data.clone()).collect::<Vec<_>>(),
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::SyncError)
}

#[derive(Clone, Debug)]
struct TimeSlottedMarketEntry {
    main_character: CharacterId,
    character_id:   CharacterId,
    structure_id:   StructureId,
    region_id:      RegionId,
    source:         String,
}
