use std::collections::HashMap;
use sqlx::PgPool;
use std::time::Instant;

use crate::Error;
use crate::parser::dogma_effects::DogmaEffect;
use crate::parser::industry_modifier_sources::{ModifyResource, Modifier};
use crate::parser::industry_target_filters::Filters;
use crate::parser::type_dogma::TypeDogma;

async fn insert_into_database(
    pool:    &PgPool,
    entries: Vec<DatabaseEntry>,
) -> Result<(), Error> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionError)?;

    tracing::debug!("Clearing structure_dogma database");
    sqlx::query!("
            DELETE FROM structure_dogma
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteStructureDogma)?;
    tracing::debug!("Clearing structure_dogma database done");

    tracing::debug!("Inserting data");
    for entry in entries {
        let type_id = entry.type_id as i32;
        let amount = entry.amount as f64;
        let categories = entry.categories
            .into_iter()
            .map(|x| x as i32)
            .collect::<Vec<_>>();
        let groups = entry.groups
            .into_iter()
            .map(|x| x as i32)
            .collect::<Vec<_>>();

        sqlx::query!("
            INSERT INTO structure_dogma
            (
                ptype_id,
                modifier,
                amount,
                categories,
                groups
            )
            VALUES
            (
                $1,
                $2::BONUS_MODIFIER,
                $3,
                $4::INTEGER[],
                $5::INTEGER[]
            )
        ",
            type_id,
            entry.modifier as _,
            amount,
            &categories,
            &groups,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertStructureDogma)?;
    }
    tracing::debug!("Inserting data done");

    Ok(())
}

pub async fn run(
    pool:                      &PgPool,
    dogma_effects:             &HashMap<usize, DogmaEffect>,
    industry_modifier_sources: &HashMap<usize, ModifyResource>,
    industry_target_filters:   &HashMap<usize, Filters>,
    type_dogma:                &HashMap<usize, TypeDogma>,
) -> Result<(), Error> {
    tracing::info!("Processing dogma");
    let start = Instant::now();

    let mut dogma_attributes = HashMap::new();
    for (id, effect) in dogma_effects {
        if effect.modifier_info.is_none() {
            continue;
        }

        for modifier in effect.modifier_info.as_ref().unwrap_or(&Vec::new()) {
            if !modifier.is_some() {
                continue;
            }

            dogma_attributes.insert(
                (*id, modifier.modified_attribute_id.unwrap()),
                modifier.modifying_attribute_id.unwrap(),
            );
        }
    }

    let mut entries = Vec::new();
    for (structure_id, modifier) in industry_modifier_sources {
        let dogma = type_dogma.get(&structure_id).unwrap();

        if let Some(x) = &modifier.manufacturing {
            let x = manufacture_modifier(
                *structure_id,
                &dogma,
                &dogma_attributes,
                &industry_target_filters,
                x
            );
            entries.extend(x);
        } else if let Some(x) = &modifier.reaction {
            let x = reaction_modifier(
                *structure_id,
                &dogma,
                &dogma_attributes,
                &industry_target_filters,
                x
            );
            entries.extend(x);
        } else {
            continue;
        };
    }

    insert_into_database(
        pool,
        entries,
    )
    .await?;

    tracing::info!(
        "Finished processing dogma, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

fn manufacture_modifier(
    structure_id:            usize,
    dogma:                   &TypeDogma,
    dogma_attributes:        &HashMap<(usize, usize), usize>,
    industry_target_filters: &HashMap<usize, Filters>,
    modifier:                &Modifier,
) -> Vec<DatabaseEntry> {
    let mut entries = Vec::new();

    // Manufacture
    let mut value = 0f32;
    let mut categories = Vec::new();
    let mut groups = Vec::new();
    if let Some(materials) = &modifier.material {
        for modifier in materials {
            for effect in dogma.effects.iter() {
                if let Some(x) = dogma_attributes.get(
                    &(effect.effect_id, modifier.attribute)
                ) {
                    value = dogma
                        .attributes
                        .iter()
                        .find(|y| y.attribute_id == *x)
                        .unwrap()
                        .value;

                    if let Some(x) = modifier.filter_id {
                        let filtered = industry_target_filters.get(&x).unwrap();
                        categories.extend(filtered.category_ids.clone());
                        groups.extend(filtered.group_ids.clone());
                    }
                } else if
                    // Raitaru
                    structure_id == 35825 ||
                    // Azbel
                    structure_id == 35826 ||
                    // Sotiyo
                    structure_id == 35827 {
                    value = -1f32;
                }
            }
        }

        entries.push(DatabaseEntry {
            type_id:    structure_id,
            modifier:   "MANUFACTURE_MATERIAL".into(),
            amount:     value * (-1f32),
            categories: categories,
            groups:     groups,
        });
    }

    let mut value = 0f32;
    let mut categories = Vec::new();
    let mut groups = Vec::new();
    if let Some(times) = &modifier.time {
        for modifier in times {
            for effect in dogma.effects.iter() {
                if let Some(x) = dogma_attributes.get(&(effect.effect_id, modifier.attribute)) {
                    value = dogma.attributes
                        .iter()
                        .find(|y| y.attribute_id == *x)
                        .unwrap()
                        .value;

                    if let Some(x) = modifier.filter_id {
                        let filtered = industry_target_filters.get(&x).unwrap();
                        categories.extend(filtered.category_ids.clone());
                        groups.extend(filtered.group_ids.clone());
                    }
                } else if structure_id == 35825 {
                    // Raitaru
                    value = -15f32;
                } else if structure_id == 35826 {
                    // Azbel
                    value = -20f32;
                } else if structure_id == 35827 {
                    // Sotiyo
                    value = -25f32;
                } else if structure_id == 47512 {
                    // 'Moreau' Fortizar
                    value = -10f32;
                } else if structure_id == 47513 {
                    // 'Draccous' Fortizar
                    value = -15f32;
                }
            }
        }

        entries.push(DatabaseEntry {
            type_id:    structure_id,
            modifier:   "MANUFACTURE_TIME".into(),
            amount:     value * (-1f32),
            categories: categories,
            groups:     groups,
        });
    }

    entries
}

fn reaction_modifier(
    structure_id:     usize,
    dogma:            &TypeDogma,
    dogma_attributes: &HashMap<(usize, usize), usize>,
    filter:           &HashMap<usize, Filters>,
    modifier:         &Modifier,
) -> Vec<DatabaseEntry> {
    let mut entries = Vec::new();

    let mut value = 0f32;
    let mut categories = Vec::new();
    let mut groups = Vec::new();
    if let Some(materials) = &modifier.material {
        for modifier in materials {
            for effect in dogma.effects.iter() {
                if let Some(x) = dogma_attributes.get(&(effect.effect_id, modifier.attribute)) {
                    value = dogma.attributes
                        .iter()
                        .find(|y| y.attribute_id == *x)
                        .unwrap()
                        .value;

                    if let Some(x) = modifier.filter_id {
                        let filtered = filter.get(&x).unwrap();
                        categories.extend(filtered.category_ids.clone());
                        groups.extend(filtered.group_ids.clone());
                    }
                }
            }
        }

        entries.push(DatabaseEntry {
            type_id:    structure_id,
            modifier:   "REACTION_MATERIAL".into(),
            amount:     value * (-1f32),
            categories: categories,
            groups:     groups,
        });
    }

    let mut value = 0f32;
    let mut categories = Vec::new();
    let mut groups = Vec::new();

    if let Some(times) = &modifier.time {
        for modifier in times {
            for effect in dogma.effects.iter() {
                if let Some(x) = dogma_attributes.get(&(effect.effect_id, modifier.attribute)) {
                    value = dogma.attributes
                        .iter()
                        .find(|y| y.attribute_id == *x)
                        .unwrap()
                        .value;

                    if let Some(x) = modifier.filter_id {
                        let filtered = filter.get(&x).unwrap();
                        categories.extend(filtered.category_ids.clone());
                        groups.extend(filtered.group_ids.clone());
                    }
                } else if structure_id == 35836 {
                    // Tatara
                    value = -25f32;
                }
            }
        }

        entries.push(DatabaseEntry {
            type_id:    structure_id,
            modifier:   "REACTION_TIME".into(),
            amount:     value * (-1f32),
            categories: categories,
            groups:     groups,
        });
    }

    entries
}

/// TypeId, Modifier, Amount, CategoryId, GroupId
///
/// Modifier = TIME, MANUFACTURE, ISK
/// CategoryId and GroupId either empty (all) or filled with specific categories
#[derive(Debug)]
pub struct DatabaseEntry {
    type_id:    usize,
    modifier:   String,
    amount:     f32,
    categories: Vec<usize>,
    groups:     Vec<usize>,
}
