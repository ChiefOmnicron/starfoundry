use sqlx::PgConnection;

use crate::{AddExcess, Error, ProjectUuid, Result};

pub async fn add_with_transaction(
    transaction:  &mut PgConnection,
    project_uuid: &ProjectUuid,
    entries:      Vec<AddExcess>,
) -> Result<()> {
    let type_ids = entries
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let quantities = entries
        .iter()
        .map(|x| x.quantity)
        .collect::<Vec<_>>();

    sqlx::query!("
        INSERT INTO project_excess
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
        &quantities,
    )
    .execute(transaction)
    .await
    .map(drop)
    .map_err(|e| Error::AddExcess(e, *project_uuid))
}
