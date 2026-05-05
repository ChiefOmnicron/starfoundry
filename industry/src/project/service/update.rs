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
                project_group_id = $3,
                orderer = $4,
                name = $5,
                note = $6,
                status = $7
            WHERE id = $1
        ",
            *project_id,
            update.sell_price,
            *update.project_group_id,
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
