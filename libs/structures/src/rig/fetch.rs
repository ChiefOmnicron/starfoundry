use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::{BonusModifier, Error, Result, StructureRig};

pub async fn fetch(
    pool:        &PgPool,
    rig_type_id: TypeId,
) -> Result<StructureRig> {
    let mut material        = None;
    let mut time            = None;
    let mut category_groups = Vec::new();

    let bonuses = sqlx::query!(r#"
            SELECT
                modifier AS "modifier!: BonusModifier",
                amount,
                categories,
                groups,
                i.name
            FROM structure_dogma
            JOIN item i ON i.type_id = ptype_id
            WHERE ptype_id = $1
        "#,
            *rig_type_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchRigBonusByTypeId(e, rig_type_id))?;

    for bonus in bonuses {
        match bonus.modifier {
            BonusModifier::ManufactureMaterial |
            BonusModifier::ReactionMaterial    => {
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

    let name = sqlx::query!("
            SELECT name
            FROM item
            WHERE type_id = $1
        ",
            *rig_type_id,
        )
        .fetch_optional(pool)
        .await
        .map(|x| {
            match x {
                Some(x) => x.name,
                _       => "".into()
            }
        })
        .map_err(|x| Error::FetchRigNameByTypeId(x, rig_type_id))?;

    Ok(StructureRig {
        type_id: rig_type_id,
        material,
        time,
        name,
        category_groups,
    })
}
