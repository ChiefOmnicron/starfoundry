use sqlx::PgPool;
use starfoundry_lib_eve_gateway::System;
use starfoundry_lib_types::SystemId;

use crate::universe::error::{UniverseError, Result};

pub async fn fetch_system_bulk(
    pool:       &PgPool,
    system_ids: Vec<SystemId>,
) -> Result<Vec<System>> {
    if system_ids.is_empty() {
        return Ok(Vec::new());
    }

    let systems = sqlx::query!("
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
            WHERE system_id = ANY($1)
        ",
            &system_ids.clone().into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(UniverseError::FetchSystemBulk)?
        .into_iter()
        .map(|x| System {
            region_id:          x.region_id.into(),
            region_name:        x.region_name,
            constellation_id:   x.constellation_id.into(),
            constellation_name: x.constellation_name,
            system_id:          x.system_id.into(),
            system_name:        x.system_name,
            security:           x.security,
            security_str:       x.security_str,
        })
        .collect::<Vec<_>>();

    Ok(
        systems
    )
}
