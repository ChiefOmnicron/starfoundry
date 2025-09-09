use sqlx::PgPool;
use starfoundry_lib_types::{ConstellationId, RegionId};
use std::collections::HashMap;
use std::time::Instant;

use crate::parser::regions::Region;
use crate::parser::systems::System;
use crate::parser::constellations::Constellation;

pub async fn run(
    pool:           &PgPool,
    regions:        HashMap<RegionId, Region>,
    constellations: HashMap<ConstellationId, Constellation>,
    systems:        Vec<System>,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Processing systems");
    let start = Instant::now();

    insert_into_database(
            &pool,
            systems,
            constellations,
            regions,
        )
        .await?;

    tracing::info!(
        "Finished processing systems, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn insert_into_database(
    pool:           &PgPool,
    systems:        Vec<System>,
    constellations: HashMap<ConstellationId, Constellation>,
    regions:        HashMap<RegionId, Region>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool
        .begin()
        .await?;

    tracing::debug!("Clearing system database");
    sqlx::query!("
            DELETE FROM system
        ")
        .execute(&mut *transaction)
        .await?;
    tracing::debug!("Clearing systems database done");

    let region_ids = systems
        .iter()
        .map(|x| *x.region_id)
        .collect::<Vec<_>>();
    let region_names = systems
        .iter()
        .map(|x| regions.get(&x.region_id).unwrap().name.clone())
        .collect::<Vec<_>>();
    let constellation_ids = systems
        .iter()
        .map(|x| *x.constellation_id)
        .collect::<Vec<_>>();
    let constellation_names = systems
        .iter()
        .map(|x| constellations.get(&x.constellation_id).unwrap().name.clone())
        .collect::<Vec<_>>();
    let system_ids = systems
        .iter()
        .map(|x| *x.system_id)
        .collect::<Vec<_>>();
    let system_names = systems
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<_>>();
    let security = systems
        .iter()
        .map(|x| x.security)
        .collect::<Vec<_>>();

    tracing::debug!("Inserting data");
    sqlx::query!("
            INSERT INTO system
            (
                region_id,
                region_name,
                constellation_id,
                constellation_name,
                system_id,
                system_name,
                security
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::VARCHAR[],
                $3::INTEGER[],
                $4::VARCHAR[],
                $5::INTEGER[],
                $6::VARCHAR[],
                $7::REAL[]
            )
        ",
            &region_ids,
            &region_names,
            &constellation_ids,
            &constellation_names,
            &system_ids,
            &system_names,
            &security,
        )
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    tracing::debug!("Inserting data done");

    Ok(())
}
