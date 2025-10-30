use sqlx::PgPool;
use starfoundry_lib_eve_gateway::StructureRigResponse;
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

use crate::structure::error::{Result, StructureError};
use crate::item::services::{fetch_category, fetch_group, fetch_item_bulk};
use crate::structure::services::BonusModifier;

pub async fn list_structure_rigs(
    pool:              &PgPool,
    structure_type_id: TypeId,
) -> Result<Vec<StructureRigResponse>> {
    let rigs = sqlx::query!(r#"
            SELECT
                type_id,
                excludes
            FROM structure_rig
            WHERE structures @> ARRAY[$1]::INTEGER[]
        "#,
            *structure_type_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StructureError::FetchStructureRigs(e, structure_type_id))?;

    let type_ids = rigs
        .iter()
        .map(|x| x.type_id.into())
        .collect::<Vec<_>>();

    let items = fetch_item_bulk(
            pool,
            type_ids,
        )
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let mut rig_result = Vec::new();
    for rig in rigs {
        let bonuses = sqlx::query!(r#"
                SELECT
                    modifier AS "modifier!: BonusModifier",
                    amount,
                    categories,
                    groups,
                    i.name
                FROM structure_dogma
                JOIN item i ON i.type_id = structure_dogma.type_id
                WHERE structure_dogma.type_id = $1
            "#,
                rig.type_id,
            )
            .fetch_all(pool)
            .await;

        let bonuses = if let Ok(x) = bonuses {
            x
        } else {
            Vec::new()
        };

        let mut material        = None;
        let mut time            = None;
        let mut categories = Vec::new();
        let mut groups     = Vec::new();

        for bonus in bonuses {
            match bonus.modifier {
                BonusModifier::ManufactureMaterial |
                BonusModifier::ReactionMaterial      => {
                    material = Some(bonus.amount as f32);
                },
                BonusModifier::ManufactureTime |
                BonusModifier::ReactionTime    => {
                    time = Some(bonus.amount as f32);
                }
            }

            for category_id in bonus.categories {
                if let Some(x) = fetch_category(
                    &pool,
                    category_id.into(),
                ).await? {
                    categories.push(x)
                }
            }

            for group_id in bonus.groups {
                if let Some(x) = fetch_group(
                    &pool,
                    group_id.into(),
                ).await? {
                    groups.push(x)
                }
            }
        }

        let item = if let Some(x) = items.get(&rig.type_id.into()) {
            x
        } else {
            continue;
        };

        rig_result.push(StructureRigResponse {
            excludes: rig.excludes.into_iter().map(Into::into).collect::<Vec<_>>(),
            item:     item.clone(),

            material,
            time,
            categories,
            groups,
        });
    }

    Ok(rig_result)
}
