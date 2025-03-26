use sqlx::PgConnection;

use crate::{AddProduct, Error, ProjectUuid, Result};

pub async fn add_with_transaction(
    transaction:  &mut PgConnection,
    project_uuid: &ProjectUuid,
    products:     Vec<AddProduct>,
) -> Result<()> {
    let type_ids = products
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let quantities = products
        .iter()
        .map(|x| x.quantity as i32)
        .collect::<Vec<_>>();
    let material_efficiency = products
        .iter()
        .map(|x| x.material_efficiency as i32)
        .collect::<Vec<_>>();
    sqlx::query!("
            INSERT INTO project_products
            (
                project_id,
                type_id,
                quantity,
                material_efficiency
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::INTEGER[]
            )
        ",
            **project_uuid,
            &type_ids,
            &quantities,
            &material_efficiency
        )
        .execute(transaction)
        .await
        .map(drop)
        .map_err(|e| Error::AddProduct(e, *project_uuid))
}
