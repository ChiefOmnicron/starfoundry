use sqlx::PgPool;
use starfoundry_lib_types::SystemId;
use std::collections::HashMap;

use crate::engine::ProjectStructureGroup;

pub async fn system_index(
    pool:             &PgPool,
    structure_groups: Vec<ProjectStructureGroup>
) -> Result<HashMap<SystemId, (f32, f32)>, Box<dyn std::error::Error>> {
    let systems = structure_groups
        .iter()
        .map(|x| x.system_ids.iter().map(|x| **x).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    let system_index = sqlx::query!("
                SELECT
                    system_id,
                    manufacturing,
                    reaction
                FROM industry_index
                WHERE timestamp = (
                    SELECT timestamp
                    FROM industry_index
                    WHERE system_id = ANY($1)
                    GROUP BY system_id, timestamp
                    ORDER BY timestamp DESC
                    LIMIT 1
                )
                AND system_id = ANY($1)
            ",
            &systems
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|x| (x.system_id.into(), (x.manufacturing, x.reaction)))
        .collect::<HashMap<_, _>>();

    Ok(system_index)
}
