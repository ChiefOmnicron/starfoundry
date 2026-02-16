use sqlx::PgPool;
use starfoundry_lib_eve_gateway::SystemIndex;
use starfoundry_lib_types::SystemId;

use crate::industry::error::{IndustryError, Result};
use crate::universe::services::fetch_system;

pub async fn fetch_system_index(
    pool:      &PgPool,
    system_id: SystemId,
) -> Result<Option<SystemIndex>> {
    let index = sqlx::query!(r#"
            SELECT
                manufacturing,
                reaction,
                copying,
                invention,
                research_time,
                research_material
            FROM system_index
            WHERE system_id = $1
            ORDER BY timestamp DESC
        "#,
            *system_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| IndustryError::FetchSystemIndex(e, system_id))?;

    if let Some(x) = index {
        let system = fetch_system(
                pool,
                system_id
            )
            .await?
            .ok_or(IndustryError::NoSystem)?;

        Ok(Some(SystemIndex {
            system:                 system,
            copying:                x.copying,
            invention:              x.invention,
            manufacturing:          x.manufacturing,
            reaction:               x.reaction,
            researching_material:   x.research_material,
            researching_time:       x.research_time,
        }))
    } else {
        Ok(None)
    }
}
