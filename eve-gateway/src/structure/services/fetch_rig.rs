use sqlx::PgPool;
use starfoundry_lib_eve_gateway::StructureRigResponse;
use starfoundry_lib_types::TypeId;

use crate::item::services::{fetch_category, fetch_group, fetch_item};
use crate::structure::error::{Result, StructureError};

pub async fn fetch_rig(
    pool:        &PgPool,
    rig_type_id: TypeId,
) -> Result<Option<StructureRigResponse>> {
    let rig = sqlx::query!(r#"
            SELECT
                type_id,
                excludes
            FROM structure_rig
            WHERE type_id = $1
        "#,
            *rig_type_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureError::FetchRigInformation(e, rig_type_id))?;

    let rig = if let Some(x) = rig {
        x
    } else {
        return Ok(None)
    };

    let item = fetch_item(
            pool,
            rig.type_id.into(),
        )
        .await?;
    let item = if let Some(x) = item {
        x
    } else {
        return Ok(None);
    };

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

    let mut material   = None;
    let mut time       = None;
    let mut categories = Vec::new();
    let mut groups     = Vec::new();

    for bonus in bonuses.iter() {
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
    }

    let mut rig_categories = bonuses
        .iter()
        .flat_map(|x| x.categories.clone())
        .map(Into::into)
        .collect::<Vec<_>>();
    rig_categories.sort();
    rig_categories.dedup();
    for category_id in rig_categories {
        if let Some(x) = fetch_category(
            &pool,
            category_id,
        ).await? {
            categories.push(x)
        }
    }

    let mut rig_groups = bonuses
        .iter()
        .flat_map(|x| x.groups.clone())
        .map(Into::into)
        .collect::<Vec<_>>();
    rig_groups.sort();
    rig_groups.dedup();
    for group_id in rig_groups {
        if let Some(x) = fetch_group(
            &pool,
            group_id,
        ).await? {
            groups.push(x)
        }
    }

    let result = StructureRigResponse {
        excludes: rig.excludes.into_iter().map(Into::into).collect::<Vec<_>>(),
        item:     item,

        material,
        time,
        categories,
        groups,
    };

    Ok(Some(result))
}

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(type_name = "BONUS_MODIFIER")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum BonusModifier {
    ManufactureMaterial,
    ManufactureTime,
    ReactionMaterial,
    ReactionTime,
}
