use sqlx::PgConnection;

use crate::{AddJobEntry, Error, ProjectUuid, Result};

pub async fn add_with_transaction(
    transaction:  &mut PgConnection,
    project_uuid: &ProjectUuid,
    entries:      Vec<AddJobEntry>,
) -> Result<()> {
    let type_ids = entries
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let runs = entries
        .iter()
        .map(|x| x.runs)
        .collect::<Vec<_>>();
    let structure_ids = entries
        .iter()
        .map(|x| *x.structure_id)
        .collect::<Vec<_>>();

    sqlx::query!("
            INSERT INTO project_job
            (
                project_id,
                type_id,
                runs,
                structure_id
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::UUID[]
            )
        ",
            **project_uuid,
            &type_ids,
            &runs,
            &structure_ids,
        )
        .execute(transaction)
        .await
        .map(drop)
        .map_err(Error::InsertJobs)
}
