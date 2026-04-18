use sqlx::PgPool;

use crate::project::{ProjectUuid, SolutionUuid};
use crate::project::error::{ProjectError, Result};

pub async fn initialize(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    solution_id:    SolutionUuid,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectError::TransactionError)?;

    sqlx::query!("
            UPDATE project
            SET
                solution_id = $2,
                status = 'READY_TO_START'
            WHERE id = $1
        ",
            *project_id,
            *solution_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Initialize)?;

    sqlx::query!("
            INSERT INTO project_market
            (
                project_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM (
                SELECT type_id, quantity
                FROM solution_material
                WHERE solution_id = $2
            )
        ",
            *project_id,
            *solution_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Initialize)?;

    sqlx::query!("
            INSERT INTO project_stock
            (
                project_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM (
                SELECT type_id, quantity
                FROM solution_stock
                WHERE solution_id = $2
            )
        ",
            *project_id,
            *solution_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Initialize)?;

    sqlx::query!("
            INSERT INTO project_excess
            (
                project_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM (
                SELECT type_id, quantity
                FROM solution_excess
                WHERE solution_id = $2
                AND quantity > 0
            )
            ON CONFLICT (project_id, type_id)
            DO UPDATE SET
                quantity = EXCLUDED.quantity
        ",
            *project_id,
            *solution_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Initialize)?;

    sqlx::query!("
            INSERT INTO project_job
            (
                project_id,
                type_id,
                runs,
                structure_id
            )
            SELECT $1, * FROM (
                SELECT type_id, runs, structure_id
                FROM solution_manufacturing
                WHERE solution_id = $2
            )
        ",
            *project_id,
            *solution_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Initialize)?;

    transaction
        .commit()
        .await
        .map_err(ProjectError::TransactionError)?;

    Ok(())
}
