use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayClient;
use starfoundry_lib_eve_gateway::market::EveGatewayApiClientMarket;
use starfoundry_lib_types::{CharacterId, RegionId, StructureId};
use starfoundry_lib_worker::Task;

use crate::{SERVICE_NAME, WorkerMarketTask};
use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::order::insert_structure_market;

pub async fn by_player_station_task(
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerMarketTask>,
) -> Result<()> {
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
    let entries = match client
        .fetch_market_by_player(
            additional_data.source,
            additional_data.character_id,
            additional_data.structure_id,
        )
        .await {

        Ok(x) => {
            x
        },
        Err(e) => {
            tracing::error!("Error while fetching market data, {:?}", e);
            task.append_error(e.to_string());
            return Err(e.into());
        }
    };

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
    character_id: CharacterId,
    structure_id: StructureId,
    region_id:    RegionId,
    // source from where the player structure was fetched
    // for example, either from the industry tool, or the appraisal
    source:       String,
}
