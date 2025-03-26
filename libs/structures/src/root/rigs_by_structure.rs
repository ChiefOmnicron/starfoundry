use sqlx::PgPool;
use starfoundry_libs_types::TypeId;

use crate::{Error, Result, StructureRig};

pub async fn rig_by_structure_type_id(
    pool:         &PgPool,
    structure_id: TypeId,
) -> Result<Vec<StructureRig>> {
    sqlx::query!(
        r#"
            SELECT
                i.type_id,
                i.name
            FROM structure_rigs sr
            JOIN items i ON i.type_id = sr.type_id
            WHERE structures @> ARRAY[$1]::INTEGER[]
            ORDER BY i.name
        "#,
            *structure_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchRigsByStructureTypeId(e, structure_id))
        .map(|x| {
            x.into_iter()
                .map(|y| {
                    StructureRig {
                        name:   y.name
                                    .replace("Standup M-Set ", "")
                                    .replace("Standup L-Set ", "")
                                    .replace("Standup XL-Set ", ""),
                        type_id: y.type_id.into(),

                        category_groups: Vec::new(),
                        material:        None,
                        time:            None,
                    }
                })
                .collect::<Vec<_>>()
        })
}
