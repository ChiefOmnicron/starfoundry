use sqlx::PgConnection;

use crate::{Error, ProjectUuid, Result, StockMinimal};

pub async fn add_with_transaction(
    transaction:  &mut PgConnection,
    project_uuid: &ProjectUuid,
    entries:      Vec<StockMinimal>,
) -> Result<()> {
    let type_ids = entries
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let quantity = entries
        .iter()
        .map(|x| x.quantity)
        .collect::<Vec<_>>();

    sqlx::query!("
        INSERT INTO project_stocks
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
        **project_uuid,
        &type_ids,
        &quantity
    )
    .execute(transaction)
    .await
    .map(drop)
    .map_err(|e| Error::AddStock(e, *project_uuid))
}
