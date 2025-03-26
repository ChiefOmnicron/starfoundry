use sqlx::PgPool;
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;
use std::time::Instant;

use crate::parser::type_material::TypeMaterial;
use crate::Error;

pub async fn run(
    pool:           &PgPool,
    type_materials: &HashMap<TypeId, Vec<TypeMaterial>>,
) -> Result<(), Error> {
    tracing::info!("Processing items");
    let start = Instant::now();

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionError)?;

    tracing::debug!("Clearing database");
    sqlx::query!("
            DELETE FROM item_reprocessing
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteItems)?;
    tracing::debug!("Clearing database done");

    let mut product_type_ids  = Vec::new();
    let mut material_type_ids = Vec::new();
    let mut quantities       = Vec::new();

    for (type_id, materials) in type_materials {
        for material in materials {
            product_type_ids.push(**type_id);
            material_type_ids.push(*material.material_type_id);
            quantities.push(material.quantity);
        }
    }

    tracing::debug!("Inserting data");
    sqlx::query!("
            INSERT INTO item_reprocessing
            (
                type_id,
                material_type_id,
                quantity
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            &product_type_ids,
            &material_type_ids,
            &quantities,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertItemReprocessing)?;
    tracing::debug!("Inserting data done");

    transaction
        .commit()
        .await
        .map_err(Error::TransactionError)?;
    tracing::debug!("Transaction commited");

    tracing::info!(
        "Finished processing item reprocessing, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}
