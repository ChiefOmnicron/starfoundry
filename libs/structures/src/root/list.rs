use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use uuid::Uuid;

use crate::{Error, Result, StructureListFilter};

pub async fn list(
    pool:         &PgPool,
    character_id: CharacterId,
    filter:       StructureListFilter,
) -> Result<Vec<Uuid>> {
    sqlx::query!(r#"
            SELECT id
            FROM structures
            WHERE
                NOT (LOWER(name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (system_id = $3) IS FALSE AND
                NOT (type_id = $4) IS FALSE AND
                NOT ($5 = ANY(services)) IS FALSE AND
                owner = $1
                ORDER BY name
        "#,
            *character_id,
            filter.name,
            filter.system_id,
            filter.structure_type_id,
            filter.service_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::ListStructureIds(e, character_id, filter))
        .map(|ids| {
            ids
                .into_iter()
                .map(|x| x.id)
                .collect::<Vec<_>>()
        })
}

