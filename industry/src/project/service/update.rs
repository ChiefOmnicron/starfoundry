use sqlx::PgPool;
use starfoundry_lib_industry::ProjectUuid;

use crate::project::error::{ProjectError, Result};
use starfoundry_lib_industry::project::UpdateProject;

pub async fn update(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    update:         UpdateProject,
) -> Result<()> {
    let result = sqlx::query!("
            UPDATE project
            SET
                sell_price = $2,
                orderer = $3,
                name = $4,
                note = $5,
                status = $6
            WHERE id = $1
        ",
            *project_id,
            update.sell_price,
            update.orderer,
            update.name,
            update.note,
            update.status as _,
        )
        .execute(pool)
        .await
        .map_err(ProjectError::Update)?;

    if result.rows_affected() == 0 {
        return Err(ProjectError::NotFound(project_id));
    }

    Ok(())
}
