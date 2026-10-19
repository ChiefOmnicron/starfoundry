use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayClient;
use starfoundry_eve_gateway::market::EveGatewayApiClientMarket;
use starfoundry_lib_types::{RegionId, StructureId};
use starfoundry_lib_worker::Task;

use crate::error::MarketWorkerError;
use crate::metric::WorkerMetric;
use crate::order::insert_structure_market;
use crate::{SERVICE_NAME, WorkerMarketTask};

pub async fn by_npc_station_task(
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerMarketTask>,
) -> Result<(), MarketWorkerError> {
    let additional_data: AdditionalData = match task.additional_data() {
        Ok(Some(x)) => x,
        Ok(None) => {
            tracing::error!("[{:?}] missing additional data", task.task);
            task.append_error("Missing additional data");
            return Err(MarketWorkerError::ParseAdditionalData)
        },
        Err(e) => {
            tracing::error!(
                "[{:?}] error parsing additional data, {}",
                task.task,
                e
            );
            task.append_error(format!("Missing additional data {}", e));
            return Err(MarketWorkerError::ParseAdditionalData)
        },
    };

    let client = EveGatewayClient::new(SERVICE_NAME)?;
    let entries = client
        .list_market_by_region(additional_data.region_id)
        .await?
        .into_iter()
        .filter(|x| *x.location_id == *additional_data.structure_id)
        .collect::<Vec<_>>();

    insert_structure_market(
        pool,
        task,
        additional_data.structure_id,
        additional_data.region_id,
        entries,
    )
    .await
}

#[derive(Debug, Deserialize)]
struct AdditionalData {
    structure_id: StructureId,
    region_id:    RegionId,
}
