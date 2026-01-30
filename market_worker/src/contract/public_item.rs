use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::contract::EveGatewayApiClientContract;
use starfoundry_lib_eve_gateway::EveGatewayClient;
use starfoundry_lib_types::ContractId;
use starfoundry_lib_worker::Task;

use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::{SERVICE_NAME, WorkerMarketTask};

pub async fn public_contract_items(
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
    let contracts = client
        .fetch_public_contract_items(additional_data.contract_id.into())
        .await?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::BeginTransaction)?;

    if contracts.is_empty() {
        return Ok(());
    }

    let mut is_included         = Vec::new();
    let mut quantity            = Vec::new();
    let mut record_id           = Vec::new();
    let mut type_id             = Vec::new();
    let mut item_id             = Vec::new();
    let mut is_blueprint_copy   = Vec::new();
    let mut material_efficiency = Vec::new();
    let mut time_efficiency     = Vec::new();
    let mut runs                = Vec::new();

    for entry in contracts.iter() {
        is_included.push(entry.is_included);
        quantity.push(entry.quantity);
        record_id.push(*entry.record_id);
        type_id.push(*entry.type_id);
        item_id.push(entry.item_id.map(|x| *x));
        is_blueprint_copy.push(entry.is_blueprint_copy);
        material_efficiency.push(entry.material_efficiency);
        time_efficiency.push(entry.time_efficiency);
        runs.push(entry.runs);
    }

    let update_start = std::time::Instant::now();
    let result = sqlx::query!("
            INSERT INTO contract_item
            (
                contract_id,
                is_included,
                is_blueprint_copy,

                quantity,

                record_id,
                type_id,
                item_id,

                material_efficiency,
                time_efficiency,
                runs
            )
            SELECT $1, * FROM UNNEST(
                $2::BOOLEAN[],
                $3::BOOLEAN[],
                $4::BIGINT[],
                $5::BIGINT[],
                $6::INTEGER[],
                $7::BIGINT[],
                $8::INTEGER[],
                $9::INTEGER[],
                $10::INTEGER[]
            )
            ON CONFLICT (contract_id, record_id)
            DO NOTHING
        ",
            *additional_data.contract_id,
            &is_included,
            &is_blueprint_copy as _,
            &quantity as _,
            &record_id,
            &type_id,
            &item_id as _,
            &material_efficiency as _,
            &time_efficiency as _,
            &runs as _,
        )
        .execute(&mut *transaction)
        .await
        .unwrap();
    // TODO: refactor to `as_millis_f64()` when https://github.com/rust-lang/rust/issues/122451 is stable
    let update_time = update_start.elapsed().as_millis();
    task.metric.increase_added_contract_count(
        result.rows_affected(),
    );
    task.metric.added_contract_duration(
        update_time,
    );
    task.append_log(format!("Updates: {}", result.rows_affected()));

    transaction
        .commit()
        .await
        .map_err(Error::BeginTransaction)?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalData {
    pub contract_id: ContractId,
}
