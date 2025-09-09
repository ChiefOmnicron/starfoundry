use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use starfoundry_lib_types::CharacterId;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    task:        &mut Task,
    pool:        &PgPool,
    credentials: &Credentials,
) -> Result<()> {
    let client = if let Some(client) = crate::utils::eve_api_client(
            credentials.clone(),
            CharacterId(0),
        )
        .await {

        client
    } else {
        task.add_error("no default credentials");
        return Err(Error::NoCredentials(0));
    };

    let entries = match client
        .industry_index()
        .await 
        .map_err(|e| Error::ApiError(e)) {
            Ok(x) => x,
            Err(e) => {
                task.add_error(e.to_string());
                return Err(Error::NoOp);
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
        let (
            i_manufacturing,
            i_reaction,
            i_copying,
            i_invention,
            i_rme,
            i_rte,
        ) = entry.index_by_activity();

        systems.push(entry.solar_system_id as i32);
        manufacturing.push(i_manufacturing);
        reaction.push(i_reaction);
        copying.push(i_copying);
        invention.push(i_invention);
        rme.push(i_rme);
        rte.push(i_rte);
    }

    sqlx::query!("
            INSERT INTO industry_index
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
            task.add_error(format!("{e}"));
            Error::InsertIndustryIndex(e)
        })?;

    Ok(())
}
