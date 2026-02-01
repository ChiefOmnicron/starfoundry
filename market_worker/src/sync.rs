use sqlx::PgPool;
use starfoundry_lib_industry::{IndustryApiClientInternal, IndustryClient, InternalStructureFilter, Structure};
use starfoundry_lib_types::CharacterId;
use starfoundry_lib_worker::Task;
use std::collections::HashMap;

use crate::{SERVICE_NAME, WorkerMarketTask};
use crate::config::Config;
use crate::error::{Error, Result};
use crate::metric::WorkerMetric;

/// Ensures that all necessary tasks are in the queue and new structures
/// are added into the rotation
pub async fn sync_task(
    pool:   &PgPool,
    task:   &mut Task<WorkerMetric, WorkerMarketTask>,
    config: Config,
) -> Result<()> {
    for (host, config) in config.hosts {
        let client = IndustryClient::new_with_address(
            SERVICE_NAME.into(),
            config.address,
        )?;

        let structure_response = match client
            .list_structures(InternalStructureFilter {
                service_id: Some(35892.into()),
            })
            .await {

            Ok(x)  => x,
            Err(e) => {
                task.append_error(e.to_string());
                HashMap::new()
            }
        };

        match sync_player_stations(
            pool,
            &structure_response,
            host.clone(),
        ).await {
            Ok(new_entries) => {
                if new_entries > 0 {
                    task.append_log(format!("added {new_entries} player markets, host: {host}"))
                }
            },
            Err(e) => task.append_error(e.to_string()),
        };

        match sync_npc_stations(
            pool,
            &structure_response,
        ).await {
            Ok(new_entries) => {
                if new_entries > 0 {
                    task.append_log(format!("added {new_entries} npc markets"))
                }
            },
            Err(e) => task.append_error(e.to_string()),
        };
    }

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
    config: Config,
) -> Result<()> {
    for (host, config) in config.hosts {
        let client = IndustryClient::new_with_address(
            SERVICE_NAME.into(),
            config.address,
        )?;

        let structure_response = match client
            .list_structures(InternalStructureFilter {
                service_id: Some(35892.into()),
            })
            .await {

            Ok(x)  => x,
            Err(e) => {
                tracing::error!("{}", e);
                HashMap::new()
            }
        };

        sync_player_stations(
            pool,
            &structure_response,
            host,
        ).await?;

        sync_npc_stations(
            pool,
            &structure_response,
        ).await?;
    }

    sync_public_contract(
        pool,
    ).await?;

    sync_misc_tasks(
        pool,
    ).await?;

    sync_private_orders(
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
    pool:               &PgPool,
    structure_response: &HashMap<CharacterId, Vec<Structure>>,
) -> Result<usize> {
    let task_name: String = WorkerMarketTask::LatestNpc.into();

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

    let mut new_entries = Vec::new();
    for (character_id, structures) in structure_response {
        if **character_id > 0 {
            continue;
        }

        for structure in structures {
            if let None = market_stations
                .iter()
                .find(|x| {
                    x.region_id == Some(*structure.system.region_id) &&
                    x.structure_id == Some(structure.structure_id)
                }) {

                let additional_data = serde_json::json!({
                    "structure_id": structure.structure_id,
                    "region_id": structure.system.region_id,
                });
                new_entries.push(additional_data);
            }
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

async fn sync_player_stations(
    pool:               &PgPool,
    structure_response: &HashMap<CharacterId, Vec<Structure>>,
    host:               String,
) -> Result<usize> {
    let task_name: String = WorkerMarketTask::LatestPlayer.into();

    let market_stations = sqlx::query!("
            SELECT
                (additional_data ->> 'structure_id')::BIGINT AS structure_id,
                (additional_data ->> 'character_id')::INTEGER AS character_id,
                (additional_data ->> 'source')::VARCHAR AS source
            FROM worker_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = $1
        ",
            &task_name,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?;

    let mut new_entries = Vec::new();
    for (character_id, structures) in structure_response {
        if **character_id == 0 {
            continue;
        }

        for structure in structures {
            if let None = market_stations
                .iter()
                .find(|x| {
                    x.character_id == Some(**character_id) &&
                    x.structure_id == Some(structure.structure_id) &&
                    x.source == Some(host.clone())
                }) {

                let additional_data = serde_json::json!({
                    "structure_id": structure.structure_id,
                    "character_id": character_id,
                    "region_id": structure.system.region_id,
                    "source": host,
                });
                new_entries.push(additional_data);
            }
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
    for corporation_id in vec![98748294] {
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
