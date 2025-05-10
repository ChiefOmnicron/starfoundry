use sqlx::PgPool;

use crate::{Error, Product, ProjectUuid, Result};

pub async fn fetch(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Vec<Product>> {
    sqlx::query_as!(
        Product,
        r#"
            SELECT
                quantity,
                material_efficiency,
                pp.type_id AS "type_id: _"
            FROM  project_product pp
            JOIN  item i ON i.type_id = pp.type_id
            WHERE pp.project_id = $1
        "#,
            *project_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchProducts(e, project_uuid))
}
