use sqlx::PgPool;
use starfoundry_lib_eve_gateway::SystemDistanceMinimal;

use crate::system::error::{Result, SystemError};

pub async fn list_distances(
    pool: &PgPool,
) -> Result<Vec<SystemDistanceMinimal>> {
    let systems = sqlx::query!(r#"
            SELECT
                system_start,
                system_end,
                distance_ly
            FROM system_distance
        "#)
        .fetch_all(pool)
        .await
        .map_err(SystemError::ListSystemDistance)?
        .into_iter()
        .map(|x| SystemDistanceMinimal {
            system_start:   x.system_start.into(),
            system_end:     x.system_end.into(),
            distance_ly:    x.distance_ly.into(),
        })
        .collect::<Vec<_>>();

    Ok(systems)
}
