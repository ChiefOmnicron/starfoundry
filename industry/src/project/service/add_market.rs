use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::project::ProjectUuid;
use crate::project::error::{ProjectError, Result};

pub async fn add_market(
    pool:       &PgPool,
    project_id: ProjectUuid,
    entries:    Vec<AddMarketEntryRequest>,
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
            INSERT INTO project_market
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
        .map_err(ProjectError::AddMarketEntry)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddMarketEntryRequest {
    pub type_id:    TypeId,
    pub quantity:   i32,
}
