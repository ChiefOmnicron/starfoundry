use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::structure::{StructureError, StructureUuid};
use crate::structure::error::Result;

pub async fn update(
    pool:         &PgPool,
    structure_id: StructureUuid,
    data:         UpdateStructure,
) -> Result<StructureUuid> {
    sqlx::query!("
            UPDATE structure
            SET
                rigs = $2,
                services = $3
            WHERE id = $1
        ",
            *structure_id,
            &data.rigs as _,
            &data.services as _,
        )
        .execute(pool)
        .await
        .map(|_| structure_id)
        .map_err(|e| StructureError::UpdateStructure(e, structure_id))
}

#[cfg(test)]
mod update_structure_test {
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    use super::UpdateStructure;
    use crate::structure::StructureUuid;

    #[sqlx::test(
        fixtures(
            path="../fixtures",
            scripts("base")
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::update(
                &pool,
                StructureUuid(Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap()),
                UpdateStructure {
                    rigs:     vec![1, 2, 3].into_iter().map(Into::into).collect::<Vec<_>>(),
                    services: vec![1, 2, 3, 4, 5].into_iter().map(Into::into).collect::<Vec<_>>(),
                }
            )
            .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(r#"
                    SELECT
                        name,
                        type_id,
                        rigs,
                        services,
                        structure_id
                    FROM structure WHERE id = '00000000-0000-0000-0000-000000000001'
                "#,
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.rigs.len(), 3);
        assert_eq!(entry.rigs, vec![1, 2, 3]);
        assert_eq!(entry.services.len(), 5);
        assert_eq!(entry.services, vec![1, 2, 3, 4, 5]);
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "rigs": [
            37275
        ],
        "services": [
            35894
        ]
    })
)]
pub struct UpdateStructure {
    /// List of all rigs that are in the structure
    pub rigs:              Vec<TypeId>,
    /// Id of the structure in-game
    pub services:          Vec<TypeId>,
}
