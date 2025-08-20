use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::task::Task;

#[derive(Debug, Deserialize, Serialize)]
struct AdditionalData {
    checksum: Option<String>,
}

pub async fn task(
    task: &mut Task,
    pool: &PgPool,
) -> Result<()> {
    // grab the additional data
    let additional_data = if let Some(x) = task.additional_data::<AdditionalData>() {
        x
    } else {
        AdditionalData {
            checksum: None
        }
    };

    let checksum = starfoundry_lib_eve_sde_parser::import_sde(
            pool,
            additional_data.checksum,
        )
        .await
        .map_err(Error::SdeError)?;

    task.set_additional_data(Some(AdditionalData {
        checksum: Some(checksum),
    }));

    Ok(())
}
