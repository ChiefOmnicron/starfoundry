use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::contract::{ContractType, EveGatewayApiClientContract};
use starfoundry_lib_eve_gateway::EveGatewayClient;
use starfoundry_lib_types::{ContractId, RegionId};
use starfoundry_lib_worker::Task;

use crate::{SERVICE_NAME, WorkerMarketTask};
use crate::contract::AdditionalData as AdditionalDataItem;
use crate::error::{Error, Result};
use crate::metric::WorkerMetric;

pub async fn public_contracts(
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
        .fetch_public_contracts(additional_data.region_id.into())
        .await?
        .into_iter()
        .filter(|x| x.typ == ContractType::ItemExchange)
        .collect::<Vec<_>>();

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::BeginTransaction)?;

    if contracts.is_empty() {
        return Ok(());
    }

    let all_ids = contracts
        .iter()
        .map(|x| *x.contract_id)
        .collect::<Vec<_>>();

    let new_ids = sqlx::query!("
            SELECT * FROM UNNEST($1::BIGINT[]) id
            EXCEPT
            SELECT contract_id FROM contract;
        ",
            &all_ids,
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.id.unwrap().into())
        .collect::<Vec<_>>();

    let mut contracts = contracts;
    contracts.sort_by(|a, b| a.contract_id.cmp(&b.contract_id));
    contracts.dedup_by_key(|x| x.contract_id);

    let mut contract_ids            = Vec::new();
    let mut date_expired            = Vec::new();
    let mut date_issued             = Vec::new();
    let mut issuer_corporation_id   = Vec::new();
    let mut issuer                  = Vec::new();
    let mut typ                     = Vec::new();
    let mut title                   = Vec::new();
    let mut price                   = Vec::new();
    let mut buyout                  = Vec::new();
    let mut collateral              = Vec::new();
    let mut days_to_complete        = Vec::new();
    let mut for_corporation         = Vec::new();
    let mut reward                  = Vec::new();
    let mut end_location_id         = Vec::new();
    let mut start_location_id       = Vec::new();
    let mut volume                  = Vec::new();

    for entry in contracts.iter() {
        contract_ids.push(*entry.contract_id);
        date_expired.push(entry.date_expired);
        date_issued.push(entry.date_issued);
        issuer_corporation_id.push(*entry.issuer_corporation_id);
        issuer.push(*entry.issuer_id);
        typ.push(entry.typ.into());
        title.push(entry.title.clone());
        price.push(entry.price);
        buyout.push(entry.buyout);
        collateral.push(entry.collateral);
        days_to_complete.push(entry.days_to_complete);
        for_corporation.push(entry.for_corporation);
        reward.push(entry.reward);
        end_location_id.push(entry.end_location_id);
        start_location_id.push(entry.start_location_id);
        volume.push(entry.volume);
    }

    let update_start = std::time::Instant::now();
    let result = sqlx::query!("
            INSERT INTO contract
            (
                contract_id,
                date_expired,
                date_issued,
                issuer_corporation_id,
                issuer_id,
                typ,

                title,

                -- item exchange
                price,

                -- auction
                buyout,

                -- hauling
                collateral,
                days_to_complete,
                for_corporation,
                reward,
                end_location_id,
                start_location_id,
                volume
            )
            SELECT * FROM UNNEST(
                $1::BIGINT[],
                $2::TIMESTAMP[],
                $3::TIMESTAMP[],
                $4::INTEGER[],
                $5::INTEGER[],
                $6::VARCHAR[],
                $7::VARCHAR[],
                $8::FLOAT[],
                $9::FLOAT[],
                $10::FLOAT[],
                $11::INTEGER[],
                $12::BOOLEAN[],
                $13::FLOAT[],
                $14::BIGINT[],
                $15::BIGINT[],
                $16::FLOAT[]
            )
            ON CONFLICT (contract_id)
            DO NOTHING
        ",
            &contract_ids,
            &date_expired,
            &date_issued,
            &issuer_corporation_id,
            &issuer,
            &typ,

            &title as _,
            &price as _,
            &buyout as _,
            &collateral as _,
            &days_to_complete as _,
            &for_corporation as _,
            &reward as _,
            &end_location_id as _,
            &start_location_id as _,
            &volume as _,
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

    // cleanup
    let expired_start = std::time::Instant::now();
    let result = sqlx::query!("
            UPDATE contract
            SET is_active = FALSE
            WHERE (
                NOT contract_id = ANY($1) OR
                date_expired < NOW()
            )
        ",
            &all_ids,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::CleanupPublicContracts)?;
    // TODO: refactor to `as_millis_f64()` when https://github.com/rust-lang/rust/issues/122451 is stable
    let expired_time = expired_start.elapsed().as_millis();
    task.metric.increase_expired_contract_count(
        result.rows_affected(),
    );
    task.metric.expired_contract_duration(
        expired_time,
    );
    task.append_log(format!("Expires: {}", result.rows_affected()));

    insert_contract_item(
        pool,
        new_ids,
        task,
    ).await.unwrap();

    transaction
        .commit()
        .await
        .map_err(Error::BeginTransaction)?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct AdditionalData {
    region_id: RegionId,
}

async fn insert_contract_item(
    pool:         &PgPool,
    contract_ids: Vec<ContractId>,
    task:         &mut Task<WorkerMetric, WorkerMarketTask>
) -> Result<()> {
    let additional_data = contract_ids
        .into_iter()
        .map(|x| Some(AdditionalDataItem {
            contract_id: x,
        }))
        .collect::<Vec<_>>();

    task.add_subtask_bulk(
            pool,
            WorkerMarketTask::PublicContractItems,
            additional_data,
        )
        .await
        .unwrap();

    Ok(())
}
