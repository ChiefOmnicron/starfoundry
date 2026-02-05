use sqlx::PgPool;
use uuid::{NoContext, Timestamp, Uuid};

use crate::Mapping;

pub async fn migrate_industry_hubs(
    postgres_source:      &PgPool,
    postgres_destination: &PgPool,
    mappings:             &mut Mapping,
) -> Result<(), Box<dyn std::error::Error>> {
    dbg!("Start - structure group");
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
        let structure_group_id = if let Some(x) = mappings.get(&structure_group.id) {
            x.clone()
        } else {
            let timestamp = Timestamp::from_unix(NoContext, structure_group.created_at.timestamp() as u64, 0);
            let structure_group_id = Uuid::new_v7(timestamp);
            mappings.insert(structure_group.id, structure_group_id);
            structure_group_id
        };

        sqlx::query!("
                INSERT INTO industry_hub
                (
                    id,
                    name,
                    owner,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (id)
                DO UPDATE SET
                    name        = EXCLUDED.name,
                    updated_at  = EXCLUDED.updated_at
            ",
                structure_group_id,
                structure_group.name,
                structure_group.owner,
                structure_group.created_at,
                structure_group.updated_at,
            )
            .execute(&mut *transaction)
            .await?;

        sqlx::query!("
                DELETE FROM industry_hub_structure
                WHERE industry_hub_id = $1
            ",
                structure_group_id,
            )
            .execute(&mut *transaction)
            .await?;
        for structure in structure_group.structure_ids {
            let structure_id = if let Some(x) = mappings.get(&structure) {
                x
            } else {
                continue;
            };

            sqlx::query!("
                    INSERT INTO industry_hub_structure
                    (
                        industry_hub_id,
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

    Ok(())
}
