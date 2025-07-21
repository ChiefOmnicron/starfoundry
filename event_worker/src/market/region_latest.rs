use chrono::Days;
use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_types::{CharacterId, RegionId, StationId};

use crate::error::{Error, Result};
use crate::task::Task;

#[derive(Debug, Deserialize)]
struct AdditionalData {
    region_id:  RegionId,
}

pub async fn task(
    task:        &mut Task,
    pool:        &PgPool,
    credentials: &Credentials,
) -> Result<()> {
    // grab the additional data
    let additional_data = if let Some(x) = task.additional_data::<AdditionalData>() {
        x
    } else {
        tracing::error!(
            "additional data was empty, but was expected to be filled, task: {:?}",
            task.task
        );
        task.add_error("additional data was empty");
        return Err(Error::NoOp);
    };

    let client = if let Some(client) = crate::utils::eve_api_client(
            credentials.clone(),
            CharacterId(0),
        )
        .await {
        client
    } else {
        // The client with CharacterId 0 will always be there, as we add him
        // when initializing the credential cache
        task.add_error("no default credentials");
        return Ok(())
    };

    let mut entries = match client
        .market_by_region(&additional_data.region_id.into())
        .await
        .map_err(|e| Error::ApiError(e)) {
            Ok(x) => x,
            Err(e) => {
                task.add_error(e.to_string());
                return Err(Error::NoOp);
            }
        };

    entries.sort_by(|a, b| a.order_id.cmp(&b.order_id));
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
            DELETE FROM market_order_latest
            WHERE
                region_id = $1 AND
                structure_id = 0
        ",
            *additional_data.region_id
        )
        .execute(&mut *transaction)
        .await
        // proper error
        .map_err(|e| Error::DeleteLatestOrders(e, StationId(*additional_data.region_id as i64)))?;

    sqlx::query!("
            INSERT INTO market_order_latest
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
            DO UPDATE SET remaining = EXCLUDED.remaining
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
        .map_err(|e| Error::InsertLatestOrders(e, StationId(*additional_data.region_id as i64)))?;

    transaction
        .commit()
        .await
        .map_err(Error::CommitTransaction)?;

    Ok(())
}
