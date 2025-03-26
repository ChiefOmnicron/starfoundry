use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, Result, StructureGroupUuid};

pub async fn list(
    pool:         &PgPool,
    character_id: CharacterId,
) -> Result<Vec<StructureGroupUuid>> {
    let ids = sqlx::query!(r#"
            SELECT id
            FROM structure_groups
            WHERE owner = $1
        "#,
            *character_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::ListGroups(e, character_id))?
        .into_iter()
        .map(|x| x.id.into())
        .collect::<Vec<_>>();

    Ok(ids)
}
