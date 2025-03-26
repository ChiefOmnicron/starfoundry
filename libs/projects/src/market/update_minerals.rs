use sqlx::PgPool;

use crate::{Error, ProjectUuid, Result, UpdateMineral};

pub async fn update_minerals(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    entries:      Vec<UpdateMineral>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionBeginError)?;

    // Just delete all minerals, we will insert them in the next step
    sqlx::query!("
            DELETE FROM project_market
            WHERE type_id = ANY(ARRAY[
                34, 35, 36, 37, 38, 39, 40
            ])
              AND project_id = $1
        ",
            *project_uuid,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::DeleteMarketMinerals(e, project_uuid))?;

    // Collect together all changes
    let type_ids = entries
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let quantities = entries
        .iter()
        .map(|x| x.quantity)
        .collect::<Vec<_>>();

    // Insert the minerals again, either raw, compressed, or both
    sqlx::query!("
            INSERT INTO project_market
            (
                project_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            *project_uuid,
            &type_ids,
            &quantities,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateMarket(e, project_uuid))?;

    transaction
        .commit()
        .await
        .map_err(Error::TransactionCommitError)?;

    Ok(())
}
