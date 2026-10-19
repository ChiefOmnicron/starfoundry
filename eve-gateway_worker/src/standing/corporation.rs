use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayClient;
use starfoundry_eve_gateway::standing::EveGatewayApiClientStanding;
use starfoundry_gateway::Identity;
use starfoundry_lib_types::{CharacterId, CorporationId};
use starfoundry_lib_worker::Task;

use crate::SERVICE_NAME;
use crate::error::{EveGatewayWorkerError, Result};
use crate::metric::WorkerMetric;
use crate::tasks::WorkerEveGatewayTask;

pub async fn corporation_standings(
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerEveGatewayTask>,
) -> Result<()> {
    let additional_data: AdditionalData = match task.additional_data() {
        Ok(Some(x)) => x,
        Ok(None) => {
            tracing::error!("[{:?}] missing additional data", task.task);
            task.append_error("Missing additional data");
            return Err(EveGatewayWorkerError::ParseAdditionalData)
        },
        Err(e) => {
            tracing::error!(
                "[{:?}] error parsing additional data, {}",
                task.task,
                e
            );
            task.append_error(format!("Missing additional data {}", e));
            return Err(EveGatewayWorkerError::ParseAdditionalData)
        },
    };

    let identity = Identity::new(
        additional_data.character_id,
        additional_data.corporation_id,
        additional_data.source,
    );
    let client = EveGatewayClient::new_with_identity(SERVICE_NAME, identity)?;
    let entries = match client
        .list_corporation_standings()
        .await
    {
        Ok(x) => x,
        Err(e) => {
            tracing::error!(
                "Error while fetching corporation standings data, {:?}",
                e
            );
            task.append_error(e.to_string());
            return Err(e.into());
        },
    };

    if entries.is_empty() {
        return Ok(());
    }

    let mut contact_ids = Vec::new();
    let mut contact_types = Vec::new();
    let mut standings = Vec::new();

    for entry in entries {
        contact_ids.push(*entry.contact_id);
        contact_types.push(entry.contact_type);
        standings.push(entry.standing);
    }

    sqlx::query!(
        "
            DELETE FROM standing
            WHERE owner_id = $1
            AND source = 'corporation'
        ",
        *additional_data.character_id,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        task.append_error(format!("{e}"));
        EveGatewayWorkerError::CleanupStandings(e)
    })?;

    sqlx::query!(
        "
            INSERT INTO standing
            (
                owner_id,
                source,
                contact_id,
                contact_type,
                standing
            )
            SELECT $1, 'corporation', * FROM UNNEST(
                $2::INTEGER[],
                $3::VARCHAR[],
                $4::REAL[]
            )
        ",
        *additional_data.character_id,
        &contact_ids,
        &contact_types,
        &standings,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        task.append_error(format!("{e}"));
        EveGatewayWorkerError::InsertStandings(e)
    })?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct AdditionalData {
    source:         String,
    character_id:   CharacterId,
    corporation_id: CorporationId,
}
