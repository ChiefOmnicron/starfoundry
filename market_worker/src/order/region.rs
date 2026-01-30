use chrono::Days;
use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayClient;
use starfoundry_lib_eve_gateway::market::EveGatewayApiClientMarket;
use starfoundry_lib_types::RegionId;
use starfoundry_lib_worker::Task;

use crate::error::Error;
use crate::metric::WorkerMetric;
use crate::{SERVICE_NAME, WorkerMarketTask};

pub async fn by_region_task(
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerMarketTask>,
) -> Result<(), Error> {
    let additional_data: AdditionalData = match task.additional_data() {
        Ok(Some(x)) => x,
        Ok(None)    => {
            tracing::error!("[{:?}] missing additional data", task.task);
            task.append_error("Missing additional data");
            return Err(Error::ParseAdditionalData)
        },
        Err(e)      => {
            tracing::error!("[{:?}] error parsing additional data, {}", task.task, e);
            task.append_error(format!("Missing additional data {}", e));
            return Err(Error::ParseAdditionalData)
        }
    };

    let client = EveGatewayClient::new(SERVICE_NAME.into())?;
    let mut entries = client
        .fetch_market_by_region(additional_data.region_id)
        .await?;

    entries.dedup_by_key(|x| x.order_id);

    let order_ids = entries
        .iter()
        .map(|x| *x.order_id as i64)
        .collect::<Vec<_>>();
    let type_id = entries
        .iter()
        .map(|x| *x.type_id as i32)
        .collect::<Vec<_>>();
    let price = entries
        .iter()
        .map(|x| x.price as f64)
        .collect::<Vec<_>>();
    let remaining = entries
        .iter()
        .map(|x| x.volume_remain as i32)
        .collect::<Vec<_>>();
    let expires = entries
        .iter()
        .map(|x| x
            .issued
            .checked_add_days(Days::new(x.duration as u64))
            .unwrap()
        )
        .collect::<Vec<_>>();
    let is_buy = entries
        .iter()
        .map(|x| x.is_buy_order.into())
        .collect::<Vec<_>>();

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::BeginTransaction)?;

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
            SELECT 0, $1, * FROM UNNEST(
                $2::BIGINT[],
                $3::INTEGER[],
                $4::INTEGER[],
                $5::FLOAT[],
                $6::TIMESTAMP[],
                $7::BOOLEAN[]
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
            *additional_data.region_id,
            &order_ids,

            &type_id,
            &remaining,
            &price,
            &expires,
            &is_buy,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::InsertRegionOrders(e, additional_data.region_id))?;

    sqlx::query!("
            DELETE FROM market_order_latest
            WHERE region_id = $1
            AND structure_id = 0
            AND (
                NOT order_id = ANY($2) OR
                remaining = 0 OR
                expires   < NOW()
            )
        ",
            *additional_data.region_id,
            &order_ids,
        )
        .execute(&mut *transaction)
        .await
        // proper error
        .map_err(|e| Error::DeleteRegionOrders(e, additional_data.region_id))?;

    transaction
        .commit()
        .await
        .map_err(Error::CommitTransaction)?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct AdditionalData {
    region_id:    RegionId,
}
