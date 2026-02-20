use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;
use std::time::Instant;

use crate::Error;
use crate::parser::blueprints::BlueprintEntry;
use crate::parser::type_ids::TypeIdEntry;

const STANDUP_MANUFACTURING_PLANT: i32 = 35878;
const STANDUP_SUPER_CAPITAL_SHIPYARD: i32 = 35877;
const STANDUP_CAPITAL_SHIPYARD: i32 = 35881;
const STANDUP_BIOCHEMICAL_REACTION: i32 = 45539;
const STANDUP_COMPOSITE_REACTION: i32 = 45537;
const STANDUP_HYBRID_REACTION: i32 = 45538;

pub async fn run(
    pool:       &PgPool,
    blueprints: &HashMap<TypeId, BlueprintEntry>,
    items:      &HashMap<TypeId, TypeIdEntry>,
) -> Result<(), Error> {
    tracing::info!("Processing blueprints");
    let start = Instant::now();

    insert_into_database(
            &pool,
            &blueprints,
            &items,
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
    items:      &HashMap<TypeId, TypeIdEntry>,
) -> Result<(), Error> {
    let type_ids = blueprints
        .iter()
        .map(|(type_id, _)| **type_id as i32)
        .collect::<Vec<_>>();
    let max_runs = blueprints
        .iter()
        .map(|(_, x)| x.max_production_limit as i32)
        .collect::<Vec<_>>();
    let service_modules = blueprints
        .iter()
        .map(|(a, v)| {
            match items.get(a) {
                Some(x) => {
                    if x.group_id == 1013.into()|| x.group_id == 110.into() {
                        STANDUP_SUPER_CAPITAL_SHIPYARD
                    } else if
                        x.group_id == 944.into() ||
                        x.group_id == 537.into() ||
                        x.group_id == 643.into() ||
                        x.group_id == 1718.into() {

                        STANDUP_CAPITAL_SHIPYARD
                    } else if x.group_id == 1890.into() {
                        STANDUP_BIOCHEMICAL_REACTION
                    } else if x.group_id == 1888.into() {
                        STANDUP_COMPOSITE_REACTION
                    } else if x.group_id == 1889.into() || x.group_id == 4097.into() {
                        STANDUP_HYBRID_REACTION
                    } else {
                        STANDUP_MANUFACTURING_PLANT
                    }
                },
                None => STANDUP_MANUFACTURING_PLANT
            }
        })
        .collect::<Vec<_>>();

    sqlx::query!("
            DELETE FROM blueprint
        ")
        .execute(pool)
        .await
        .unwrap();

    sqlx::query!("
            INSERT INTO blueprint
            (
                type_id,
                max_runs,
                required_service_type_id
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            &type_ids,
            &max_runs,
            &service_modules,
        )
        .execute(pool)
        .await
        .unwrap();

    Ok(())
}
