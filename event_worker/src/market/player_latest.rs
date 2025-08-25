use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_types::{CharacterId, RegionId, StationId};

use crate::error::{Error, Result};
use crate::market::insert::insert_structure_market;
use crate::task::Task;
use crate::utils::additional_data;

#[derive(Debug, Deserialize)]
struct AdditionalData {
    structure_id: StationId,
    region_id:    RegionId,
    owner_id:     CharacterId,
}

pub async fn task(
    task:        &mut Task,
    pool:        &PgPool,
    credentials: &Credentials,
) -> Result<()> {
    let additional_data = additional_data::<AdditionalData>(task)?;

    let client = if let Some(client) = crate::utils::eve_api_client(
            credentials.clone(),
            additional_data.owner_id,
        )
        .await {
        client
    } else {
        // The client with CharacterId 0 will always be there, as we add him
        // when initializing the credential cache
        task.add_error("no default credentials");
        return Ok(())
    };

    let entries = match client
        .market_by_structure(&additional_data.structure_id.into())
        .await
        .map_err(|e| Error::ApiError(e)) {
            Ok(x) => x,
            Err(e) => {
                task.add_error(e.to_string());
                return Err(Error::NoOp);
            }
        };

    insert_structure_market(
        pool,
        task,
        additional_data.structure_id,
        additional_data.region_id,
        entries
    ).await?;

    Ok(())
}
