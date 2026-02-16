use chrono::Days;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::eve_market::Market;
use starfoundry_lib_worker::Task;

use crate::WorkerMarketTask;
use crate::error::{Error, Result};
use crate::metric::WorkerMetric;

pub async fn insert_private_orders(
    pool:       &PgPool,
    task:       &mut Task<WorkerMetric, WorkerMarketTask>,
    issuer_id:  i32,
    entries:    Vec<Market>,
) -> Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut entries = entries;
    entries.sort_by(|a, b| a.order_id.cmp(&b.order_id));
    entries.dedup_by_key(|x| x.order_id);

    let mut order_ids       = Vec::new();
    let mut structure_ids   = Vec::new();
    let mut type_id         = Vec::new();
    let mut price           = Vec::new();
    let mut remaining       = Vec::new();
    let mut expires         = Vec::new();
    let mut is_buy          = Vec::new();

    for entry in entries {
        structure_ids.push(*entry.location_id);
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
            INSERT INTO market_private_order AS mol
            (
                issuer_id,
                order_id,
                structure_id,

                type_id,
                remaining,
                price,
                expires,
                is_buy
            )
            SELECT $1, * FROM UNNEST(
                $2::BIGINT[],
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
            issuer_id,
            &order_ids,
            &structure_ids,

            &type_id,
            &remaining,
            &price,
            &expires,
            &is_buy,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertPrivateOrders)?;
    // TODO: refactor to `as_millis_f64()` when https://github.com/rust-lang/rust/issues/122451 is stable
    let update_time = update_start.elapsed().as_millis();
    task.metric.increase_private_orders_added_count(
        issuer_id,
        result.rows_affected(),
    );
    task.metric.add_private_orders_added_duration(
        issuer_id,
        update_time,
    );
    task.append_log(format!("Updates: {}", result.rows_affected()));

    sqlx::query!("
            INSERT INTO market_private_order_history (
                order_id,
                remaining
            )
            SELECT * FROM UNNEST(
                $1::BIGINT[],
                $2::INTEGER[]
            )
            ON CONFLICT (order_id, remaining)
            DO NOTHING
        ",
            &order_ids,
            &remaining,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertPrivateHistoryOrders)?;

    transaction
        .commit()
        .await
        .map_err(Error::CommitTransaction)?;

    Ok(())
}
