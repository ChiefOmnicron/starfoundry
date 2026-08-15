use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{System, SystemDistance};
use starfoundry_lib_types::SystemId;

use crate::system::error::{Result, SystemError};

pub async fn list_in_range(
    pool:               &PgPool,
    start_system_id:    SystemId,
) -> Result<Vec<SystemDistance>> {
    let systems = sqlx::query!(r#"
            SELECT
                -- start system
                ss.region_id AS "start_region_id",
                ss.region_name AS "start_region_name",
                ss.constellation_id AS "start_constellation_id",
                ss.constellation_name AS "start_constellation_name",
                ss.system_id AS "start_system_id",
                ss.system_name AS "start_system_name",
                ss.security AS "start_security",
                ss.security_str AS "start_security_str",

                -- end system
                se.region_id AS "end_region_id",
                se.region_name AS "end_region_name",
                se.constellation_id AS "end_constellation_id",
                se.constellation_name AS "end_constellation_name",
                se.system_id AS "end_system_id",
                se.system_name AS "end_system_name",
                se.security AS "end_security",
                se.security_str AS "end_security_str",

                sd.distance_ly
            FROM system_distance sd
            JOIN system ss ON ss.system_id = sd.system_start
            JOIN system se ON se.system_id = sd.system_end
            WHERE sd.system_start = $1
        "#,
            *start_system_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| SystemError::ListSystemsInRange(e, start_system_id))?
        .into_iter()
        .map(|x| {
            let start_system = System {
                region_id:          x.start_region_id.into(),
                region_name:        x.start_region_name,
                constellation_id:   x.start_constellation_id.into(),
                constellation_name: x.start_constellation_name,
                system_id:          x.start_system_id.into(),
                system_name:        x.start_system_name,
                security:           x.start_security,
                security_str:       x.start_security_str,
            };
            let end_system = System {
                region_id:          x.end_region_id.into(),
                region_name:        x.end_region_name,
                constellation_id:   x.end_constellation_id.into(),
                constellation_name: x.end_constellation_name,
                system_id:          x.end_system_id.into(),
                system_name:        x.end_system_name,
                security:           x.end_security,
                security_str:       x.end_security_str,
            };

            SystemDistance {
                system_start:   start_system,
                system_end:     end_system,
                distance_ly:    x.distance_ly.into(),
            }
        })
        .collect::<Vec<_>>();
    Ok(systems)
}
