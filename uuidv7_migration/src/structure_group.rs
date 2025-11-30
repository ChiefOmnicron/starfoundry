use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::Mapping;

pub async fn migrate_structure_group(
    postgres_source:      &PgPool,
    postgres_destination: &PgPool,
    structure_mapping:    &Mapping,
) -> Result<Mapping, Box<dyn std::error::Error>> {
    dbg!("Start - structure group");
    let mut mappings = HashMap::new();

    let structure_groups = sqlx::query!(r#"
            SELECT
                id,
                name,
                owner,
                structure_ids,
                created_at,
                updated_at
            FROM structure_group
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut transaction = postgres_destination.begin().await?;
    for structure_group in structure_groups {
        let structure_group_id = Uuid::now_v7();
        mappings.insert(structure_group.id, structure_group_id);

        sqlx::query!("
                INSERT INTO structure_group
                (
                    id,
                    name,
                    owner,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5)
            ",
                structure_group_id,
                structure_group.name,
                structure_group.owner,
                structure_group.created_at,
                structure_group.updated_at,
            )
            .execute(&mut *transaction)
            .await?;

        for structure in structure_group.structure_ids {
            let structure_id = if let Some(x) = structure_mapping.get(&structure) {
                x
            } else {
                continue;
            };

            sqlx::query!("
                    INSERT INTO structure_group_structure
                    (
                        structure_group_id,
                        structure_id
                    )
                    VALUES ($1, $2)
                ",
                    structure_group_id,
                    structure_id,
                )
                .execute(&mut *transaction)
                .await?;
        }
    }
    transaction.commit().await?;
    dbg!("Done - structure group");

    Ok(mappings)
}
