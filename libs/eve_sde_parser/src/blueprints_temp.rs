use sqlx::PgPool;
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;
use std::time::Instant;

use crate::Error;
use crate::parser::blueprints::BlueprintEntry;

pub async fn run(
    pool:       &PgPool,
    blueprints: &HashMap<TypeId, BlueprintEntry>,
) -> Result<(), Error> {
    tracing::info!("Processing blueprints");
    let start = Instant::now();

    insert_into_database(
            &pool,
            &blueprints,
        )
        .await?;

    tracing::info!(
        "Finished processing blueprints, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

pub async fn insert_into_database(
    pool:       &PgPool,
    blueprints: &HashMap<TypeId, BlueprintEntry>,
) -> Result<(), Error> {
    let type_ids = blueprints
        .iter()
        .map(|(type_id, _)| **type_id as i32)
        .collect::<Vec<_>>();
    let max_runs = blueprints
        .iter()
        .map(|(_, x)| x.max_production_limit as i32)
        .collect::<Vec<_>>();

    sqlx::query!("
            DELETE FROM blueprints_temp
        ")
        .execute(pool)
        .await
        .unwrap();

    sqlx::query!("
            INSERT INTO blueprints_temp
            (
                type_id,
                max_runs
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[]
            )
        ",
            &type_ids,
            &max_runs,
        )
        .execute(pool)
        .await
        .unwrap();

    Ok(())
}
