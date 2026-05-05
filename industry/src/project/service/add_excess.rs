use sqlx::PgPool;
use starfoundry_lib_industry::ProjectUuid;

use crate::project::error::{ProjectError, Result};
use starfoundry_lib_industry::project::AddExcessEntryRequest;

pub async fn add_excess(
    pool:       &PgPool,
    project_id: ProjectUuid,
    entries:    Vec<AddExcessEntryRequest>,
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
            *project_id,
            &type_ids,
            &quantities,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(ProjectError::AddExcessEntry)
}
