use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientMarket, EveGatewayClient};
use starfoundry_lib_types::{CharacterId, CorporationId, RegionId, StructureId};
use starfoundry_lib_worker::Task;

use crate::{SERVICE_NAME, WorkerMarketTask};
use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::order::insert_structure_market;
use starfoundry_lib_gateway::Identity;

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

    let identity = Identity::new(
        additional_data.character_id,
        additional_data.corporation_id,
        additional_data.source,
    );
    let client = EveGatewayClient::new_with_identity(SERVICE_NAME, identity)?;
    let entries = match client
        .list_market_by_player(
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
        RegionId(0),
        entries
    )
    .await
}

#[derive(Debug, Deserialize)]
struct AdditionalData {
    character_id:   CharacterId,
    corporation_id: CorporationId,
    structure_id:   StructureId,
    // source from where the player structure was fetched
    // for example, either from the industry tool, or the appraisal
    source:         String,
}
