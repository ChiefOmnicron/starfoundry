use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayClient;
use starfoundry_lib_eve_gateway::eve_market::EveGatewayApiClientEveMarket;
use starfoundry_lib_types::{RegionId, StructureId};
use starfoundry_lib_worker::Task;

use crate::{SERVICE_NAME, WorkerMarketTask};
use crate::error::Error;
use crate::metric::WorkerMetric;
use crate::order::insert_structure_market;

pub async fn by_npc_station_task(
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
    let entries = client
        .fetch_market_by_region(additional_data.region_id)
        .await?
        .into_iter()
        .filter(|x| *x.location_id == *additional_data.structure_id)
        .collect::<Vec<_>>();

    insert_structure_market(
        &pool,
        task,
        additional_data.structure_id,
        additional_data.region_id, 
        entries
    )
    .await
}

#[derive(Debug, Deserialize)]
struct AdditionalData {
    structure_id: StructureId,
    region_id:    RegionId,
}
