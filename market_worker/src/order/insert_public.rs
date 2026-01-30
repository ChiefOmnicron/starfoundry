use chrono::Days;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::market::Market;
use starfoundry_lib_types::{RegionId, StructureId};
use starfoundry_lib_worker::Task;

use crate::WorkerMarketTask;
use crate::error::{Error, Result};
use crate::metric::WorkerMetric;

pub async fn insert_structure_market(
    pool:         &PgPool,
    task:         &mut Task<WorkerMetric, WorkerMarketTask>,
    structure_id: StructureId,
    region_id:    RegionId,
    entries:      Vec<Market>,
) -> Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut entries = entries;
    entries.sort_by(|a, b| a.order_id.cmp(&b.order_id));
    entries.dedup_by_key(|x| x.order_id);

    let mut order_ids  = Vec::new();
    let mut type_id    = Vec::new();
    let mut price      = Vec::new();
    let mut remaining  = Vec::new();
    let mut expires    = Vec::new();
    let mut is_buy     = Vec::new();

    for entry in entries {
        if *entry.location_id != *structure_id {
            continue;
        }

        order_ids.push(*entry.order_id as i64);
        type_id.push(*entry.type_id as i32);
        price.push(entry.price as f64);
        remaining.push(entry.volume_remain as i32);
        expires.push(
            entry.issued
                .checked_add_days(Days::new(entry.duration as u64))
                .unwrap()
        );
        is_buy.push(entry.is_buy_order.into());
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::BeginTransaction)?;

    let update_start = std::time::Instant::now();
    let result = sqlx::query!("
            INSERT INTO market_order_latest AS mol
            (
                structure_id,
                region_id,
                order_id,

                type_id,
                remaining,
                price,
                expires,
                is_buy
            )
            SELECT $1, $2, * FROM UNNEST(
                $3::BIGINT[],
                $4::INTEGER[],
                $5::INTEGER[],
                $6::FLOAT[],
                $7::TIMESTAMP[],
                $8::BOOLEAN[]
            )
            ON CONFLICT (order_id)
            DO UPDATE SET
                remaining = EXCLUDED.remaining,
                expires = EXCLUDED.expires,
                price = EXCLUDED.price
            WHERE mol.remaining != EXCLUDED.remaining
            OR mol.expires != EXCLUDED.expires
            OR mol.price != EXCLUDED.price
        ",
            *structure_id,
            *region_id,
            &order_ids,

            &type_id,
            &remaining,
            &price,
            &expires,
            &is_buy,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::InsertStationOrdersError(e, structure_id))?;
    // TODO: refactor to `as_millis_f64()` when https://github.com/rust-lang/rust/issues/122451 is stable
    let update_time = update_start.elapsed().as_millis();
    task.metric.increase_market_order_rows_changed(
        structure_id,
        result.rows_affected(),
    );
    task.metric.add_market_order_latest_update_duration(
        structure_id,
        update_time,
    );
    task.append_log(format!("Updates: {}", result.rows_affected()));

    let delete_start = std::time::Instant::now();
    let result = sqlx::query!("
            DELETE FROM market_order_latest
            WHERE structure_id = $1
            AND (
                NOT order_id = ANY($2) OR
                remaining = 0 OR
                expires < NOW()
            )
        ",
            *structure_id,
            &order_ids,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::CleanupOrdersError(e, structure_id))?;
    // TODO: refactor to `as_millis_f64()` when https://github.com/rust-lang/rust/issues/122451 is stable
    let delete_time = delete_start.elapsed().as_millis();
    task.metric.increase_market_order_rows_deleted(
        structure_id,
        result.rows_affected(),
    );
    task.metric.add_market_order_latest_delete_duration(
        structure_id,
        delete_time,
    );
    task.append_log(format!("Deletes: {}", result.rows_affected()));

    transaction
        .commit()
        .await
        .map_err(Error::CommitTransaction)?;

    Ok(())
}
