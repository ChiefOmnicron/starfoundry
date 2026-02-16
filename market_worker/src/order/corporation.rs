use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayClient;
use starfoundry_lib_eve_gateway::eve_market::EveGatewayApiClientEveMarket;
use starfoundry_lib_types::{CharacterId, CorporationId};
use starfoundry_lib_worker::Task;

use crate::{SERVICE_NAME, WorkerMarketTask};
use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::order::insert_private_orders;

// FIXME: this needs to be a config file, the whole syncing needs to be a config file
const HOST_INDUSTRY: &str = "industry.dev.starfoundry.space";

pub async fn corporation_orders(
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
        .fetch_corporation_orders(
            HOST_INDUSTRY.into(),
            additional_data.character_id,
            additional_data.corporation_id,
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

    match insert_private_orders(
            &pool,
            task,
            *additional_data.corporation_id,
            entries
        )
        .await {
            Ok(_)   => Ok(()),
            Err(e)  => {
                task.append_error(e.to_string());
                Err(e.into())
            },
        }
}

#[derive(Debug, Deserialize)]
struct AdditionalData {
    corporation_id: CorporationId,
    character_id:   CharacterId,
}
