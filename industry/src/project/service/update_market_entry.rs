use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::project::{MarketUuid, ProjectUuid};
use crate::project::error::ProjectError;
use crate::project::error::Result;

pub async fn update_market_entry(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    market_id:      MarketUuid,
    update:         UpdateMarketEntryRequest,
) -> Result<()> {
    sqlx::query!(r#"
            UPDATE project_market
            SET
                quantity = $3,
                cost = $4,
                source = $5
            WHERE project_id = $1
            AND id = $2
        "#,
            *project_id,
            *market_id,
            update.quantity,
            update.cost,
            update.source,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(ProjectError::Update)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMarketEntryRequest {
    quantity:   i32,
    cost:       Option<f64>,
    source:     Option<String>,
}
