use sqlx::{PgConnection, PgPool};

use crate::{Error, AddMarket, ProjectUuid, Result};

pub async fn add(
    pool:         &PgPool,
    project_uuid: &ProjectUuid,
    entry:        AddMarket,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionBeginError)?;

    add_with_transaction(&mut transaction, project_uuid, vec![entry]).await?;

    transaction
        .commit()
        .await
        .map_err(Error::TransactionCommitError)?;

    Ok(())
}

pub async fn add_with_transaction(
    transaction: &mut PgConnection,
    project_uuid: &ProjectUuid,
    entries:      Vec<AddMarket>,
) -> Result<()> {
    let type_ids = entries
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let quantities = entries
        .iter()
        .map(|x| x.quantity)
        .collect::<Vec<_>>();
    let cost = entries
        .iter()
        .map(|x| x.cost)
        .collect::<Vec<_>>();
    let sources = entries
        .iter()
        .map(|x| x.source.clone())
        .collect::<Vec<_>>();

    sqlx::query!("
            INSERT INTO project_market
            (
                project_id,
                type_id,
                quantity,
                cost,
                source
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::FLOAT[],
                $5::VARCHAR[]
            )
        ",
            **project_uuid,
            &type_ids,
            &quantities,
            &cost as _,
            &sources as _,
        )
        .execute(transaction)
        .await
        .map(drop)
        .map_err(|e| Error::AddMarket(e, *project_uuid))
}
