use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_types::CharacterId;

use crate::error::{Error, Result};
use crate::task::Task;

#[derive(Debug, Deserialize)]
struct AdditionalData {
    character_id: CharacterId,
}

pub async fn task(
    task:        &mut Task,
    pool:        &PgPool,
    credentials: &Credentials
) -> Result<()> {
    // grab the additional data
    let additional_data = if let Some(x) = task.additional_data::<AdditionalData>() {
        x
    } else {
        tracing::error!(
            "additional data was empty, but was expected to be filled, task: {:?}",
            task.task
        );
        task.add_error("additional data was empty");
        return Err(Error::NoOp);
    };

    // create an eve api client
    let client = if let Some(client) = crate::utils::eve_api_client(
            credentials.clone(),
            additional_data.character_id,
        ).await {

        client
    } else {
        return Err(Error::NoCredentials(*additional_data.character_id));
    };

    // fetch all industry jobs for the corporation
    let industry_jobs = match client
        .character_blueprints()
        .await
        .map_err(|e| Error::ApiError(e)) {
            Ok(x) => x,
            Err(e) => {
                task.add_error(e.to_string());
                return Err(Error::NoOp);
            }
        };

    if industry_jobs.is_empty() {
        return Ok(());
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::BeginTransaction)?;

    sqlx::query!("
            DELETE FROM asset_blueprint
            WHERE owner_id = $1
        ",
            *additional_data.character_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteAssetBlueprints)?;

    let mut type_id             = Vec::new();
    let mut quantity            = Vec::new();
    let mut runs                = Vec::new();
    let mut material_efficiency = Vec::new();
    let mut time_efficiency     = Vec::new();

    for entry in industry_jobs.iter() {
        type_id.push(*entry.type_id);
        quantity.push(entry.quantity);
        runs.push(entry.runs);
        material_efficiency.push(entry.material_efficiency as i32);
        time_efficiency.push(entry.time_efficiency as i32);
    }

    sqlx::query!("
            INSERT INTO asset_blueprint
            (
                owner_id,
                type_id,
                quantity,
                runs,
                material_efficiency,
                time_efficiency
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::INTEGER[],
                $5::INTEGER[],
                $6::INTEGER[]
            )
        ",
            *additional_data.character_id,
            &type_id,
            &quantity,
            &runs,
            &material_efficiency,
            &time_efficiency,
        )
        .execute(&mut *transaction)
        .await
        .map(drop)
        .map_err(Error::InsertAssetBlueprints)?;

    transaction
        .commit()
        .await
        .map_err(Error::CommitTransaction)?;

    Ok(())
}
