use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientIndustry, EveGatewayClient};
use starfoundry_lib_worker::Task;

use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::SERVICE_NAME;
use crate::tasks::WorkerEveGatewayTask;

pub async fn system_index(
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerEveGatewayTask>,
) -> Result<()> {
    let client = EveGatewayClient::new(SERVICE_NAME.into())?;
    let entries = match client
        .eve_fetch_system_index()
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

    if entries.is_empty() {
        return Ok(());
    }

    let mut systems       = Vec::new();
    let mut manufacturing = Vec::new();
    let mut reaction      = Vec::new();
    let mut copying       = Vec::new();
    let mut invention     = Vec::new();
    let mut rme           = Vec::new();
    let mut rte           = Vec::new();

    for entry in entries {
        let activity = entry.index_by_activity();

        systems.push(entry.solar_system_id as i32);
        manufacturing.push(activity.manufacturing);
        reaction.push(activity.reaction);
        copying.push(activity.copying);
        invention.push(activity.invention);
        rme.push(activity.researching_material);
        rte.push(activity.researching_time);
    }

    sqlx::query!("
            INSERT INTO system_index
            (
                system_id,
                manufacturing,
                reaction,
                copying,
                invention,
                research_time,
                research_material
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::REAL[],
                $3::REAL[],
                $4::REAL[],
                $5::REAL[],
                $6::REAL[],
                $7::REAL[]
            )
        ",
            &systems,
            &manufacturing,
            &reaction,
            &copying,
            &invention,
            &rme,
            &rte,
        )
        .execute(pool)
        .await
        .map_err(|e| {
            task.append_error(format!("{e}"));
            Error::InsertSystemIndex(e)
        })?;

    Ok(())
}
