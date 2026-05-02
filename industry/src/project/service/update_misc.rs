use serde::Deserialize;
use starfoundry_lib_industry::ProjectUuid;
use utoipa::ToSchema;
use sqlx::PgPool;

use crate::project::error::{ProjectError, Result};

pub async fn update_misc(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    update:         Vec<UpdateMiscRequest>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectError::TransactionError)?;

    sqlx::query!("
            DELETE FROM project_misc
            WHERE project_id = $1
        ",
            *project_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(ProjectError::Update)?;

    let items = update
        .iter()
        .map(|x| x.item.clone())
        .collect::<Vec<_>>();
    let quantity = update
        .iter()
        .map(|x| x.quantity)
        .collect::<Vec<_>>();
    let description = update
        .iter()
        .map(|x| x.description.clone())
        .collect::<Vec<_>>();
    let cost = update
        .iter()
        .map(|x| x.cost as f64)
        .collect::<Vec<_>>();

    sqlx::query!("
            INSERT INTO project_misc
            (
                project_id,
                item,
                quantity,
                description,
                cost
            )
            SELECT $1, * FROM UNNEST(
                $2::VARCHAR[],
                $3::INTEGER[],
                $4::VARCHAR[],
                $5::DOUBLE PRECISION[]
            )
        ",
            *project_id,
            &items,
            &quantity as _,
            &description as _,
            &cost,
        )
        .execute(pool)
        .await
        .map_err(ProjectError::Update)?;

    transaction
        .commit()
        .await
        .map(drop)
        .map_err(ProjectError::Update)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMiscRequest {
    pub item:           String,
    pub cost:           f32,

    pub description:    Option<String>,
    pub quantity:       Option<i32>,
}
