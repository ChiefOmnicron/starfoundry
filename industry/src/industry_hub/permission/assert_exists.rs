use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::AppState;
use crate::industry_hub::error::{IndustryHubError, Result};
use crate::industry_hub::IndustryHubUuid;

pub async fn assert_exists(
    State(state):       State<AppState>,
    Path(structure_id): Path<IndustryHubUuid>,
    request:            Request,
    next:               Next,
) -> Result<impl IntoResponse> {
    assert_exists_check(
            &state.pool,
            structure_id,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_exists_check(
    pool:              &PgPool,
    industry_hub_uuid: IndustryHubUuid,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT id
            FROM industry_hub
            WHERE id = $1
        ",
            *industry_hub_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| IndustryHubError::FetchPermission(e, industry_hub_uuid))?;

    if result.is_some() {
        Ok(())
    } else {
        Err(IndustryHubError::NotFound(industry_hub_uuid))
    }
}
