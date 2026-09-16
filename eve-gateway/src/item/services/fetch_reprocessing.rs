use sqlx::PgPool;
use starfoundry_lib_eve_gateway::Reprocessing;
use starfoundry_lib_types::TypeId;

use crate::item::error::{ItemError, Result};

pub async fn fetch_reprocessing(
    pool:    &PgPool,
    type_id: TypeId,
) -> Result<Vec<Reprocessing>> {
    let entries = sqlx::query!(r#"
            SELECT
                type_id,
                material_type_id,
                quantity
            FROM item_reprocessing
            WHERE type_id = $1
        "#,
            *type_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ItemError::FetchReprocessing(e, type_id))?
        .into_iter()
        .map(|x| Reprocessing {
            type_id:    x.material_type_id.into(),
            quantity:   x.quantity,
        })
        .collect::<Vec<_>>();

    Ok(entries)
}
