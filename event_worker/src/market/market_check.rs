use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    task: &mut Task,
    pool: &PgPool,
) -> Result<()> {
    match market_npc(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added {new_entries} NPC markets"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    match market_player(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added {new_entries} player markets"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    match market_prices(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added {new_entries} market prices"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    Ok(())
}

async fn market_npc(
    pool: &PgPool
) -> Result<usize> {
    let market_stations_target = vec![
        // Jita
        (10000002, 60003760),
        // Amarr
        (10000043, 60008494),
    ];

    let market_stations_is = sqlx::query!("
            SELECT
                (additional_data ->> 'region_id')::INTEGER AS region_id,
                (additional_data ->> 'structure_id')::INTEGER AS structure_id
            FROM event_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = 'MARKET_LATEST_NPC'
        ")
        .fetch_all(pool)
        .await
        .map_err(Error::FetchMarketNpcQueue)
        .map(|x| {
            x
                .iter()
                .map(|y| (
                    y.region_id.unwrap_or_default(),
                    y.structure_id.unwrap_or_default(),
                ))
                .collect::<Vec<_>>()
        })?;

    let mut new_entries = Vec::new();
    for market_station in market_stations_target {
        if !market_stations_is.contains(&market_station) {
            let additional_data = serde_json::json!({
                "region_id": market_station.0,
                "structure_id": market_station.1,
            });
            new_entries.push(additional_data);
        }
    }

    tracing::info!("Added {} new npc market jobs", new_entries.len());

    sqlx::query!("
            INSERT INTO event_queue (task, additional_data)
            SELECT 'MARKET_LATEST_NPC', * FROM UNNEST(
                $1::JSONB[]
            )
        ",
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::InsertNewJobs)
}

async fn market_player(
    pool: &PgPool
) -> Result<usize> {
    struct MarketStation {
        structure_id: i64,
        owner_id: i32,
    }

    let market_stations_target = sqlx::query!("
            SELECT
                owner,
                structure_id,
                region_id
            FROM structures s
            JOIN systems sys ON s.system_id = sys.system_id
            -- filter for standup market hub I
            WHERE 35892 = ANY(services)
        ")
        .fetch_all(pool)
        .await
        .map_err(Error::FetchMarketstationsPlayer)?
        .into_iter()
        .map(|x| {
            MarketStation {
                owner_id: x.owner,
                structure_id: x.structure_id,
            }
        })
        .collect::<Vec<_>>();

    let market_stations_is = sqlx::query!("
            SELECT
                (additional_data ->> 'structure_id')::BIGINT AS structure_id,
                (additional_data ->> 'owner_id')::INTEGER AS owner_id
            FROM event_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = 'MARKET_LATEST_PLAYER'
        ")
        .fetch_all(pool)
        .await
        .map_err(Error::FetchMarketPlayerQueue)
        .map(|x| {
            x
                .iter()
                .map(|y| MarketStation {
                    owner_id: y.owner_id.unwrap_or_default(),
                    structure_id: y.structure_id.unwrap_or_default(),
                })
                .collect::<Vec<_>>()
        })?;

    let mut new_entries = Vec::new();
    for market_station in market_stations_target {
        if market_stations_is
            .iter()
            .find(|x| x.structure_id == market_station.structure_id)
            .is_none() {

            let additional_data = serde_json::json!({
                "structure_id": market_station.structure_id,
                "owner_id": market_station.owner_id,
            });
            new_entries.push(additional_data);
        }
    }

    tracing::info!("Added {} new player market jobs", new_entries.len());

    sqlx::query!("
            INSERT INTO event_queue (task, additional_data)
            SELECT 'MARKET_LATEST_PLAYER', * FROM UNNEST(
                $1::JSONB[]
            )
        ",
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::InsertNewJobs)
}

async fn market_prices(
    pool: &PgPool
) -> Result<usize> {
    let has_entry = sqlx::query!("
            SELECT COUNT(1) AS count
            FROM event_queue
            WHERE task = 'MARKET_PRICES'
        ")
        .fetch_one(pool)
        .await
        .map(|x| x.count.unwrap_or_default() > 0)
        .map_err(Error::FetchTask)?;

    if has_entry {
        return Ok(0);
    }

    sqlx::query!("
        INSERT INTO event_queue (task)
        VALUES ('MARKET_PRICES')
    ")
    .execute(pool)
    .await
    .map(|_| 1)
    .map_err(Error::InsertNewJobs)
}
