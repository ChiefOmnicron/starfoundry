use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, Result, StructureDynamicGroupUuid};

pub async fn list(
    pool:         &PgPool,
    character_id: CharacterId,
) -> Result<Vec<StructureDynamicGroupUuid>> {
    let ids = sqlx::query!(r#"
            SELECT id
            FROM structure_dynamic_group
            WHERE owner = $1
        "#,
            *character_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::ListGroupIds(e, character_id))?
        .into_iter()
        .map(|x| x.id.into())
        .collect::<Vec<_>>();
    Ok(ids)
}
