use sqlx::PgPool;
use starfoundry_lib_types::SystemId;

use crate::universe::fetch::model::System;
use crate::universe::error::{Result, UniverseError};

pub async fn fetch(
    pool:      &PgPool,
    system_id: SystemId,
) -> Result<Option<System>> {
    let system = sqlx::query!("
            SELECT
                region_id,
                region_name,
                constellation_id,
                constellation_name,
                system_id,
                system_name,
                security,
                security_str
            FROM system
            WHERE system_id = $1
        ",
            *system_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| UniverseError::FetchSystem(e, system_id))?;

    if let Some(x) = system {
        Ok(Some(System {
            region_id:          x.region_id.into(),
            region_name:        x.region_name,
            constellation_id:   x.constellation_id.into(),
            constellation_name: x.constellation_name,
            system_id:          x.system_id.into(),
            system_name:        x.system_name,
            security:           x.security,
            security_str:       x.security_str,
        }))
    } else {
        Ok(None)
    }
}
