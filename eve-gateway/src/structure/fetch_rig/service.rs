use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::item::fetch::fetch_item;
use crate::structure::error::{Result, StructureError};
use crate::structure::list_structure_rigs::{BonusModifier, StructureRigResponse};

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

    let mut material        = None;
    let mut time            = None;
    let mut category_groups = Vec::new();
    for bonus in bonuses {
        match bonus.modifier {
            BonusModifier::ManufacturingMaterial |
            BonusModifier::ReactionMaterial      => {
                material = Some(bonus.amount as f32);
            },
            BonusModifier::ManufactureTime |
            BonusModifier::ReactionTime    => {
                time = Some(bonus.amount as f32);
            }
        }

        if category_groups.is_empty() {
            let mut cg = Vec::new();
            cg.extend(bonus.categories);
            cg.extend(bonus.groups);
            category_groups = cg;
        }
    }

    let result = StructureRigResponse {
        excludes: rig.excludes.into_iter().map(Into::into).collect::<Vec<_>>(),
        item:     item,

        material,
        time,
        category_groups,
    };

    Ok(Some(result))
}
