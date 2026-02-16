use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientEveAsset, EveGatewayClient};
use starfoundry_lib_types::CharacterId;
use starfoundry_lib_worker::Task;

use crate::asset::insert::insert_assets;
use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::SERVICE_NAME;
use crate::tasks::WorkerEveGatewayTask;

pub async fn assets(
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerEveGatewayTask>,
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
        .eve_fetch_character_assets(
            additional_data.source,
            additional_data.character_id,
        )
        .await {

        Ok(x) => {
            x
        },
        Err(e) => {
            tracing::error!("Error while fetching corporation blueprint data, {:?}", e);
            task.append_error(e.to_string());
            return Err(e.into());
        }
    };

    match insert_assets(
            &pool,
            task,
            *additional_data.character_id,
            entries,
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
    source:       String,
    character_id: CharacterId,
}
