use sqlx::PgPool;
use starfoundry_lib_types::SystemId;
use std::time::Instant;

use crate::Error;
use crate::parser::systems::System;
use std::collections::HashSet;

pub async fn run(
    pool:       &PgPool,
    systems:    &Vec<System>,
) -> Result<(), Error> {
    tracing::info!("Processing system distances");
    let start = Instant::now();

    let mut calculated_systems = HashSet::new();
    let mut system_distance = Vec::new();
    for system_a in systems.iter() {
        for system_b in systems.iter() {
            if system_a.system_id == system_b.system_id {
                continue;
            }

            // target is HS
            if system_b.security >= 0.45f32 {
                continue;
            }

            let distance = (
                (system_a.position.x - system_b.position.x).powf(2.0) +
                (system_a.position.y - system_b.position.y).powf(2.0) +
                (system_a.position.z - system_b.position.z).powf(2.0)
            ).sqrt();

            if distance <= 10f64 * 9_460_000_000_000_000f64 {
                let distance = distance / 9_460_000_000_000_000f64;
                let distance = (distance * 1_000f64).round() / 1_000f64;
                system_distance.push(SystemDistance {
                    distance_ly:        distance,
                    system_id_start:    system_a.system_id,
                    system_id_end:      system_b.system_id,
                });
            }

            calculated_systems.insert((system_a.system_id, system_b.system_id));
        }
    }

    insert_into_database(
            &pool,
            system_distance,
        )
        .await?;

    tracing::info!(
        "Finished processing system distance, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn insert_into_database(
    pool:           &PgPool,
    systems:        Vec<SystemDistance>,
) -> Result<(), Error> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionError)?;

    tracing::debug!("Clearing system_distance database");
    sqlx::query!("
            DELETE FROM system_distance
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteSystems)?;
    tracing::debug!("Clearing system_distance database done");

    let distance_ly = systems
        .iter()
        .map(|x| x.distance_ly)
        .collect::<Vec<_>>();
    let system_id_start = systems
        .iter()
        .map(|x| *x.system_id_start)
        .collect::<Vec<_>>();
    let system_id_end = systems
        .iter()
        .map(|x| *x.system_id_end)
        .collect::<Vec<_>>();

    tracing::debug!("Inserting data");
    sqlx::query!("
            INSERT INTO system_distance
            (
                system_start,
                system_end,
                distance_ly
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::DOUBLE PRECISION[]
            )
        ",
            &system_id_start,
            &system_id_end,
            &distance_ly,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertSystem)?;

    transaction
        .commit()
        .await
        .map_err(Error::TransactionError)?;
    tracing::debug!("Inserting data done");

    Ok(())
}

struct SystemDistance {
    system_id_start:    SystemId,
    system_id_end:      SystemId,
    distance_ly:        f64,
}
