use sqlx::PgPool;
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;
use std::time::Instant;

use crate::Error;
use crate::parser::blueprints::BlueprintEntry;
use crate::parser::type_ids::TypeIdEntry;

pub async fn run(
    pool:       &PgPool,
    blueprints: &HashMap<TypeId, BlueprintEntry>,
    type_ids:   &HashMap<TypeId, TypeIdEntry>,
) -> Result<(), Error> {
    tracing::info!("Processing blueprint json");
    let start = Instant::now();

    insert_into_database(
            &pool,
            &blueprints,
            &type_ids
        )
        .await?;

    tracing::info!(
        "Finished processing blueprint json, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn insert_into_database(
    pool:       &PgPool,
    blueprints: &HashMap<TypeId, BlueprintEntry>,
    type_ids:   &HashMap<TypeId, TypeIdEntry>,
) -> Result<(), Error> {
    let products = crate::parser::blueprints::product_type_id_as_key(
        &blueprints,
        &type_ids,
    );

    let find_btype_id = |ptype_id: TypeId| {
        blueprints
            .iter()
            .filter(|(_, x)| x.product().is_some())
            .find(|(_, x)| x.product().unwrap() == ptype_id)
            .map(|(y, _)| y)
            .unwrap()
            .clone()
    };

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionError)?;

    tracing::debug!("Clearing blueprint_dependencies database");
    sqlx::query!("
            DELETE FROM blueprint_dependencies
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteBlueprintJson)?;
    tracing::debug!("Clearing blueprint_dependencies database done");

    tracing::debug!("Inserting data");
    for (ptype_id, pentry) in products.iter() {
        let btype_id = *find_btype_id(*ptype_id);
        let ptype_id = **ptype_id;
        let time = pentry.manufacture_time().unwrap() as i32;
        let depends_on = pentry
            .materials()
            .into_iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();

        sqlx::query!("
            INSERT INTO blueprint_dependencies
            (
                btype_id,
                ptype_id,
                time,
                depends_on
            )
            VALUES
            (
                $1::INTEGER,
                $2::INTEGER,
                $3::INTEGER,
                $4::INTEGER[]
            )
        ",
            &btype_id,
            &ptype_id,
            &time,
            &depends_on
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertBlueprintJson)?;
    }

    transaction
        .commit()
        .await
        .map_err(Error::TransactionError)?;
    tracing::debug!("Transaction commited");
    Ok(())
}
