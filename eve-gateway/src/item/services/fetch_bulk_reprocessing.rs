use sqlx::PgPool;
use starfoundry_lib_eve_gateway::Reprocessing;
use starfoundry_lib_types::TypeId;

use crate::item::error::{ItemError, Result};
use std::collections::HashMap;

pub async fn fetch_bulk_reprocessing(
    pool:       &PgPool,
    type_ids:   Vec<TypeId>,
) -> Result<HashMap<TypeId, Vec<Reprocessing>>> {
    let entries = sqlx::query!(r#"
            SELECT
                type_id,
                material_type_id,
                quantity
            FROM item_reprocessing
            WHERE type_id = ANY($1)
        "#,
            &*type_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ItemError::FetchBulkReprocessing(e))?;

    let mut result = HashMap::new();
    for entry in entries {
        let reprocessing = Reprocessing {
            type_id:    entry.material_type_id.into(),
            quantity:   entry.quantity,
        };

        result
            .entry(entry.type_id.into())
            .and_modify(|x: &mut Vec<Reprocessing>| x.push(reprocessing.clone()))
            .or_insert(vec![reprocessing]);
    }

    Ok(result)
}
