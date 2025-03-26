use sqlx::PgPool;

use crate::{Error, ProjectUuid, Result, UpdateMarket};

pub async fn update_bulk(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    updates:      Vec<UpdateMarket>,
) -> Result<()> {
    let type_ids = updates
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let quantities = updates
        .iter()
        .map(|x| x.quantity)
        .collect::<Vec<_>>();
    let cost = updates
        .iter()
        .map(|x| x.cost)
        .collect::<Vec<_>>();
    let sources = updates
        .iter()
        .map(|x| x.source.clone())
        .collect::<Vec<_>>();

    sqlx::query!("
            UPDATE project_market
              SET quantity   = data.quantity,
                  cost       = data.cost,
                  source     = data.source
            FROM (
                SELECT
                    UNNEST($2::INTEGER[]) AS type_id,
                    UNNEST($3::INTEGER[]) AS quantity,
                    UNNEST($4::REAL[]) AS cost,
                    UNNEST($5::VARCHAR[]) AS source
            ) AS data
            WHERE project_id = $1
              AND project_market.type_id = data.type_id
        ",
            *project_uuid,
            &type_ids,
            &quantities,
            &cost as _,
            &sources as _,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateMarket(e, project_uuid))
}
