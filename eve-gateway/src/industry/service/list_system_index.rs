use chrono::NaiveTime;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{SystemIndexHistory, TimeSpanQuery};
use starfoundry_lib_types::SystemId;

use crate::industry::error::{IndustryError, Result};
use crate::system::services::fetch_system;

pub async fn list_system_index(
    pool:       &PgPool,
    system_id:  SystemId,
    time_span:  TimeSpanQuery,
) -> Result<Vec<SystemIndexHistory>> {
    let index = sqlx::query!(r#"
            SELECT
                timestamp,
                manufacturing,
                reaction,
                copying,
                invention,
                research_time,
                research_material
            FROM system_index
            WHERE system_id = $1
            AND timestamp >= $2
            AND timestamp <= $3
            ORDER BY timestamp DESC
        "#,
            *system_id,
            time_span.start.and_time(NaiveTime::default()),
            time_span.end.and_time(NaiveTime::default()),
        )
        .fetch_all(pool)
        .await
        .map_err(|e| IndustryError::ListSystemIndex(e, system_id))?;

    let system = fetch_system(
            pool,
            system_id
        )
        .await?
        .ok_or(IndustryError::NoSystem)?;

    let index = index
        .into_iter()
        .map(|x| SystemIndexHistory {
            system:                 system.clone(),
            copying:                x.copying,
            invention:              x.invention,
            manufacturing:          x.manufacturing,
            reaction:               x.reaction,
            researching_material:   x.research_material,
            researching_time:       x.research_time,
            timestamp:              x.timestamp,
        })
        .collect::<Vec<_>>();
    Ok(index)
}
