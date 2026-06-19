use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientAsset, EveGatewayClient};
use starfoundry_lib_gateway::Identity;
use starfoundry_lib_types::{CharacterId, CorporationId};
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

    let identity = Identity::new(
        additional_data.character_id,
        additional_data.corporation_id,
        additional_data.source,
    );

    let client = EveGatewayClient::new_with_identity(SERVICE_NAME, identity)?;
    let entries = match client
        .list_character_assets()
        .await {

        Ok(x) => {
            x
        },
        Err(e) => {
            tracing::error!("Error while fetching character blueprint data, {:?}", e);
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
    source:         String,
    character_id:   CharacterId,
    corporation_id: CorporationId,
}
