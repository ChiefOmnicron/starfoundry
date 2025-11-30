use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::AppState;
use crate::structure_group::error::{StructureGroupError, Result};
use crate::structure_group::StructureGroupUuid;

pub async fn assert_exists(
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureGroupUuid>,
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
    pool:                 &PgPool,
    structure_group_uuid: StructureGroupUuid,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT id
            FROM structure_group
            WHERE id = $1
        ",
            *structure_group_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureGroupError::FetchPermission(e, structure_group_uuid))?;

    if result.is_some() {
        Ok(())
    } else {
        Err(StructureGroupError::NotFound(structure_group_uuid))
    }
}
