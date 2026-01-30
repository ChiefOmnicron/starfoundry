use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{ListItemFilter, StructureRigBlueprintBonus};
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};

use crate::structure::error::{Result, StructureError};
use crate::structure::services::BonusModifier;
use crate::item::services::list_items;
use std::collections::HashMap;

pub async fn fetch_rig_blueprint_bonus(
    pool:         &PgPool,
    rig_type_ids: Vec<TypeId>,
) -> Result<Vec<StructureRigBlueprintBonus>> {
    let mut structure_rig_blueprint_entries = HashMap::new();

    for rig_type_id in rig_type_ids {
        let entries = sqlx::query!(r#"
                SELECT
                    type_id,
                    modifier AS "modifier!: BonusModifier",
                    amount,
                    categories,
                    groups
                FROM structure_dogma
                WHERE type_id = $1
            "#,
                *rig_type_id,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| StructureError::FetchRigInformation(e, rig_type_id))?;

        for entry in entries.iter() {
            if !(entry.categories.len() > 0 || entry.groups.len() > 0) {
                continue;
            }

            let mut bonus_me = 0f64;
            let mut bonus_te = 0f64;
            let mut is_manufacturing = false;
            let mut is_reaction = false;

            match entry.modifier {
                BonusModifier::ManufactureMaterial => {
                    bonus_me = entry.amount;
                    is_manufacturing = true;
                },
                BonusModifier::ManufactureTime => {
                    bonus_te = entry.amount;
                    is_manufacturing = true;
                },
                BonusModifier::ReactionMaterial => {
                    bonus_me = entry.amount;
                    is_reaction = true;
                },
                BonusModifier::ReactionTime => {
                    bonus_te = entry.amount;
                    is_reaction = true;
                }
            };

            let categories: Vec<CategoryId> = entry
                .categories
                .iter()
                .map(|x| x.into())
                .collect::<Vec<_>>();
            let groups: Vec<GroupId> = entry
                .groups
                .iter()
                .map(|x| x.into())
                .collect::<Vec<_>>();

            let categories = if categories.is_empty() {
                None
            } else {
                Some(categories)
            };
            let groups = if groups.is_empty() {
                None
            } else {
                Some(groups)
            };

            list_items(
                    pool,
                    ListItemFilter {
                        categories: categories,
                        groups:     groups,
                        buildable:  Some(true),
                        limit:      Some(500),
                        ..Default::default()
                    }
                )
                .await?
                .into_iter()
                .for_each(|x| {
                    structure_rig_blueprint_entries
                        .entry(x.type_id)
                        .and_modify(|x: &mut StructureRigBlueprintBonus| {
                            if x.bonus_me == 0f64 {
                                x.bonus_me = bonus_me;
                            }
                            if x.bonus_te == 0f64 {
                                x.bonus_te = bonus_te;
                            }
                        })
                        .or_insert(StructureRigBlueprintBonus {
                            bonus_me,
                            bonus_te,
                            is_manufacturing,
                            is_reaction,
                            blueprint: x,
                        });
                });
        }
    }

    let values =structure_rig_blueprint_entries
        .values()
        .cloned()
        .into_iter()
        .collect::<Vec<_>>();
    Ok(values)
}
