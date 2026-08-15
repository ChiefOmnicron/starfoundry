use sqlx::PgPool;
use starfoundry_lib_eve_gateway::System;

use crate::system::error::{Result, SystemError};

pub async fn list(
    pool: &PgPool,
) -> Result<Vec<System>> {
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
        ")
        .fetch_all(pool)
        .await
        .map_err(SystemError::ListSystem)?
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

    Ok(systems)
}
