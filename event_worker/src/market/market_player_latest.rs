use chrono::Days;
use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_types::{CharacterId, StationId};

use crate::error::{Error, Result};
use crate::task::Task;

#[derive(Debug, Deserialize)]
struct AdditionalData {
    structure_id: StationId,
    owner_id:     CharacterId,
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
            additional_data.owner_id,
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
        .market_by_structure(&additional_data.structure_id.into())
        .await
        .map_err(|e| Error::ApiError(e)) {
            Ok(x) => x,
            Err(e) => {
                task.add_error(e.to_string());
                return Err(Error::NoOp);
            }
        };

    let mut order_ids  = Vec::new();
    let mut type_id    = Vec::new();
    let mut price      = Vec::new();
    let mut remaining  = Vec::new();
    let mut expires    = Vec::new();
    let mut is_buy     = Vec::new();

    entries.sort_by(|a, b| a.order_id.cmp(&b.order_id));
    entries.dedup_by_key(|x| x.order_id);

    for entry in entries {
        if *entry.location_id != *additional_data.structure_id {
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
            DELETE FROM market_orders_latest
            WHERE structure_id = $1
        ",
            *additional_data.structure_id
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::DeleteLatestOrders(e, additional_data.structure_id))?;

    sqlx::query!("
            INSERT INTO market_orders_latest
            (
                structure_id,
                order_id,

                type_id,
                remaining,
                price,
                expires,
                is_buy
            )
            SELECT $1, * FROM UNNEST(
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
            *additional_data.structure_id,
            &order_ids,

            &type_id,
            &remaining,
            &price,
            &expires,
            &is_buy,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::InsertLatestOrders(e, additional_data.structure_id))?;

    transaction
        .commit()
        .await
        .map_err(Error::CommitTransaction)?;

    Ok(())
}
