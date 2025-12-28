use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientMarket, EveGatewayClient};
use starfoundry_lib_types::{CharacterId, RegionId, StructureId};
use starfoundry_lib_worker::Task;

use crate::error::Error;
use crate::insert::insert_structure_market;
use crate::metric::WorkerMetric;
use crate::WorkerMarketTask;

pub async fn by_player_station(
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

    let client = EveGatewayClient::new("STARFOUNDRY_WORKER_MARKET".into()).unwrap();
    let entries = client
        .fetch_market_by_player(
            additional_data.source,
            additional_data.character_id,
            additional_data.structure_id,
        )
        .await?;

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
