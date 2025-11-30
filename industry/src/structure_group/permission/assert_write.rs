use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::AppState;
use crate::structure_group::error::{StructureGroupError, Result};
use crate::structure_group::StructureGroupUuid;

pub async fn assert_write(
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureGroupUuid>,
    identity:           ExtractIdentity,
    request:            Request,
    next:               Next,
) -> Result<impl IntoResponse> {
    assert_write_check(
            &state.pool,
            structure_id,
            identity.character_id,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_write_check(
    pool:           &PgPool,
    structure_uuid: StructureGroupUuid,
    character_id:   CharacterId,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT id
            FROM structure_group
            WHERE id = $1
            AND owner = $2
        ",
            *structure_uuid,
            *character_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureGroupError::FetchPermission(e, structure_uuid))?;

    if result.is_none() {
        return Err(StructureGroupError::Forbidden(structure_uuid, character_id));
    } else {
        Ok(())
    }
}
