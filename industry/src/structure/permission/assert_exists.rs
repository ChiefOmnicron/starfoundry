use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_industry::StructureUuid;

use crate::AppState;
use crate::structure::error::{StructureError, Result};

pub async fn assert_exists(
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureUuid>,
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
    pool:           &PgPool,
    structure_uuid: StructureUuid,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT id
            FROM structure
            WHERE id = $1
        ",
            *structure_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureError::FetchPermission(e, structure_uuid))?;

    tracing::info!("{:?} {:?}", &result, structure_uuid);
    if result.is_some() {
        Ok(())
    } else {
        Err(StructureError::NotFound(structure_uuid))
    }
}
