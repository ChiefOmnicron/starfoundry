use chrono::Days;
use sqlx::PgPool;
use starfoundry_libs_eve_api::Market;
use starfoundry_libs_types::{RegionId, StationId};

use crate::error::{Error, Result};

pub async fn insert_structure_market(
    pool:         &PgPool,
    structure_id: StationId,
    region_id:    RegionId,
    entries:      Vec<Market>,
) -> Result<()> {
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

    sqlx::query!("
            UPDATE market_order_latest
            SET touched = FALSE
            WHERE structure_id = $1
        ",
            *structure_id
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::UpdateTouchedStructure(e, structure_id))?;

    sqlx::query!("
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
        .map_err(|e| Error::InsertLatestOrdersStation(e, structure_id))?;

    sqlx::query!("
            DELETE FROM market_order_latest
            WHERE structure_id = $1
            AND (
                order_id != ANY($2) OR
                remaining = 0 OR
                expires   < NOW()
            )
        ",
            *structure_id,
            &order_ids,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::DeleteLatestOrders(e, structure_id))?;

    transaction
        .commit()
        .await
        .map_err(Error::CommitTransaction)?;

    Ok(())
}
