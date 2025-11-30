use uuid::Uuid;
use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::structure_group::StructureGroupUuid;
use crate::structure_group::error::{Result, StructureGroupError};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub async fn update(
    pool:                   &PgPool,
    character_id:           CharacterId,
    structure_group_uuid:   StructureGroupUuid,
    structure_group:        UpdateStructureGroup,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(StructureGroupError::BeginTransactionError)?;

    sqlx::query!("
            UPDATE structure_group
            SET
                name = $3
            WHERE
                id = $1 AND
                owner = $2
        ",
            *structure_group_uuid,
            *character_id,
            structure_group.name,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| StructureGroupError::UpdateStructureGroup(e, structure_group_uuid))?;

    sqlx::query!("
            DELETE FROM structure_group_structure
            WHERE structure_group_id = $1
        ",
            *structure_group_uuid,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| StructureGroupError::UpdateStructureGroup(e, structure_group_uuid))?;

    sqlx::query!("
            INSERT INTO structure_group_structure
            (
                structure_group_id,
                structure_id
            )
            SELECT $1, * FROM UNNEST($2::UUID[])
        ",
            *structure_group_uuid,
            &structure_group.structures,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| StructureGroupError::UpdateStructureGroup(e, structure_group_uuid))?;

    transaction
        .commit()
        .await
        .map(drop)
        .map_err(StructureGroupError::CommitTransactionError)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::structure_group::service::UpdateStructureGroup;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn happy_path_all(
        pool: PgPool,
    ) {
        let result = super::update(
            &pool,
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-100000000001").unwrap().into(),
            UpdateStructureGroup {
                name: "My structure group".into(),
                structures: vec![
                    Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap()
                ],
            }
        )
        .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(r#"
                    SELECT
                        name
                    FROM structure_group
                    WHERE id = '00000000-0000-0000-0000-100000000001'
                "#,
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My structure group");

        let entries = sqlx::query!(r#"
                    SELECT structure_id
                    FROM structure_group_structure
                    WHERE structure_group_id = '00000000-0000-0000-0000-100000000001'
                "#,
            )
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x| x.structure_id)
            .collect::<Vec<_>>();
        assert_eq!(entries.len(), 1);
    }
}


#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool structure group",
        "structures": [
            "019a462f-dd0a-7d07-9516-df377ae11395",
            "019aa90f-4e83-71b3-ad36-2844f1d2701a"
        ]
    })
)]
pub struct UpdateStructureGroup {
    pub name:       String,
    pub structures: Vec<Uuid>,
}
