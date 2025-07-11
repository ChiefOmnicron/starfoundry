use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

use super::SearchError;

#[instrument(err, skip(pool), level = "error")]
pub async fn systems(
    pool:  &PgPool,
    query: SystemQuery,
) -> Result<Vec<System>, SearchError> {
    let systems = sqlx::query!("
            SELECT *
            FROM system
            WHERE
                ($1 AND LOWER(system_name) LIKE LOWER($2)) OR
                system_id = $3
            LIMIT 10
        ",
            query.name.is_some(),
            format!("%{}%", query.name.unwrap_or_default()),
            query.system_id,
        )
        .fetch_all(pool)
        .await
        .map_err(SearchError::SearchSystems)?
        .into_iter()
        .map(|x| {
            System {
                region_name: x.region_name,
                region_id:   x.region_id,

                system_name: x.system_name,
                system_id:   x.system_id,
                security:    x.security,
            }
        })
        .collect::<Vec<_>>();

    Ok(systems)
}

#[derive(Debug, Deserialize)]
pub struct SystemQuery {
    name:      Option<String>,
    system_id: Option<i32>,
}


#[derive(Debug, Serialize)]
pub struct System {
    region_name: String,
    region_id:   i32,

    system_name: String,
    system_id:   i32,
    security:    f32,
}
