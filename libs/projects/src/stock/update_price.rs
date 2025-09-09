use sqlx::PgPool;
use starfoundry_lib_appraisal::AppraisalEntry;

use crate::{appraisal, Error, ProjectUuid, Result, UpdateStockPrice};
use super::fetch;

pub async fn update_price(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    update:       UpdateStockPrice
) -> Result<()> {
    let entries = fetch(pool, project_uuid)
        .await?
        .into_inner()
        .into_iter()
        .map(|x| {
            AppraisalEntry {
                name:     x.item_name,
                type_id:  x.type_id,
                quantity: x.quantity,
            }
        })
        .collect::<Vec<_>>();

    let entries = appraisal(
            &pool,
            update.appraisal,
            entries
        )
        .await?;

    let type_ids = entries
        .iter()
        .map(|(type_id, _)| **type_id)
        .collect::<Vec<_>>();
    let costs = entries
        .iter()
        .map(|(_, cost)| *cost)
        .collect::<Vec<_>>();

    sqlx::query!("
            UPDATE project_stock
            SET cost = data.cost
            FROM (
                SELECT
                    UNNEST($2::INTEGER[]) AS type_id,
                    UNNEST($3::REAL[]) AS cost
            ) AS data
            WHERE project_id = $1
              AND project_stock.type_id = data.type_id
        ",
            *project_uuid,
            &type_ids,
            &costs
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateStockPrice(e, project_uuid))?;

    Ok(())
}
