use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    task: &mut Task,
    pool: &PgPool,
) -> Result<()> {
    match blueprint_stock(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added {new_entries} blueprint stocks"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    Ok(())
}

pub async fn blueprint_stock(
    pool: &PgPool
) -> Result<usize> {
    let mut blueprint_stock_ids = sqlx::query!("
            SELECT id
            FROM stock_blueprints
        ")
        .fetch_all(pool)
        .await
        .map_err(Error::FetchBpcStockIds)
        .map(|x| {
            x
                .iter()
                .map(|y| y.id)
                .collect::<Vec<_>>()
        })?;

    let blueprint_stock_ids_is = sqlx::query!("
            SELECT (additional_data ->> 'bpc_stock_id')::UUID AS bpc_stock_id
            FROM event_queue
            WHERE (additional_data ->> 'bpc_stock_id')::UUID = ANY($1)
            AND (status = 'WAITING' OR status = 'IN_PROGRESS')
        ",
            &blueprint_stock_ids,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::FetchCharacterIdsQueue)
        .map(|x| {
            x
                .iter()
                .map(|y| y.bpc_stock_id.unwrap_or_default())
                .collect::<Vec<_>>()
        })?;

    if blueprint_stock_ids_is.len() == blueprint_stock_ids.len() {
        return Ok(0);
    }

    blueprint_stock_ids.sort();
    blueprint_stock_ids.dedup();
    let mut new_entries = Vec::new();
    for bpc_stock_id in blueprint_stock_ids {
        if !blueprint_stock_ids_is.contains(&bpc_stock_id) {
            let additional_data = serde_json::json!({
                "bpc_stock_id": bpc_stock_id,
            });
            new_entries.push(additional_data);
        }
    }

    tracing::info!("Added {} new bpc_stock jobs", new_entries.len());

    sqlx::query!("
        INSERT INTO event_queue (task, additional_data)
        SELECT 'STOCK_BLUEPRINT', * FROM UNNEST(
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
