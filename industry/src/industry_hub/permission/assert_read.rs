use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::AppState;
use crate::industry_hub::error::{IndustryHubError, Result};
use crate::industry_hub::IndustryHubUuid;

pub async fn assert_read(
    State(state):       State<AppState>,
    Path(structure_id): Path<IndustryHubUuid>,
    identity:           ExtractIdentity,
    request:            Request,
    next:               Next,
) -> Result<impl IntoResponse> {
    assert_read_check(
            &state.pool,
            structure_id,
            identity.character_id,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_read_check(
    pool:           &PgPool,
    structure_uuid: IndustryHubUuid,
    character_id:   CharacterId,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT id
            FROM industry_hub
            WHERE id = $1
            AND owner = $2
        ",
            *structure_uuid,
            *character_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| IndustryHubError::FetchPermission(e, structure_uuid))?;

    if result.is_none() {
        return Err(IndustryHubError::Forbidden(structure_uuid, character_id));
    } else {
        Ok(())
    }
}
