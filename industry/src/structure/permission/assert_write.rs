use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::AppState;
use crate::structure::error::{StructureError, Result};
use crate::structure::StructureUuid;

pub async fn assert_write(
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureUuid>,
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
    structure_uuid: StructureUuid,
    character_id:   CharacterId,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT id
            FROM structure
            WHERE id = $1
            AND owner = $2
        ",
            *structure_uuid,
            *character_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureError::FetchPermission(e, structure_uuid))?;

    if result.is_none() {
        return Err(StructureError::Forbidden(structure_uuid, character_id));
    } else {
        Ok(())
    }
}
