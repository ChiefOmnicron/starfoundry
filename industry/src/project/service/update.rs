use sqlx::PgPool;
use starfoundry_lib_industry::ProjectUuid;

use crate::project::error::{ProjectError, Result};
use starfoundry_lib_industry::project::UpdateProject;

pub async fn update(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    update:         UpdateProject,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectError::TransactionError)?;

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
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Update)?;

    if result.rows_affected() == 0 {
        return Err(ProjectError::NotFound(project_id));
    }

    sqlx::query!("
            DELETE FROM project_tag
            WHERE project_id = $1
        ",
            *project_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Update)?;

    sqlx::query!("
            INSERT INTO project_tag
            (
                project_id,
                tag_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[]
            )
        ",
            *project_id,
            &update.tags.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Update)?;

    transaction
        .commit()
        .await
        .map_err(ProjectError::TransactionError)?;

    Ok(())
}
