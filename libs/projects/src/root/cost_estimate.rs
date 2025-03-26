use sqlx::PgPool;
use starfoundry_libs_appraisal::AppraisalEntry;
use starfoundry_libs_types::{CharacterId, TypeId};
use std::collections::HashMap;

use crate::{appraisal, group_structures, AddExcess, AppraisalList, BlueprintBonus, BlueprintTyp, CostEstimateConfiguration, CostEstimateResponse, Error, MarketPrice, Result};
use crate::engine::{CalculationEngine, Dependency, EngineResult, ProjectConfigBuilder};

use super::ExcessCostEstimateEntry;

// TODO: refactor this
pub async fn cost_estimate(
    pool:         &PgPool,
    character_id: CharacterId,
    project_data: CostEstimateConfiguration,
) -> Result<CostEstimateResponse> {
    let blueprint_overwrites = project_data
        .products
        .iter()
        .map(|x| (
            x.type_id,
            BlueprintBonus {
                ptype_id: x.type_id,
                material: x.material_efficiency.unwrap_or_default() as f32,
                time:     0f32,
            }
        ))
        .collect::<HashMap<_, _>>();

    let structure_groups = group_structures(
        pool,
        character_id,
        project_data.structure_group,
    ).await?;

    let mut systems = structure_groups
        .iter()
        .map(|x| x.system_ids.iter().map(|x| **x).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    systems.sort();
    systems.dedup();

    let mut system_index = HashMap::new();
    for system in systems {
        let index = sqlx::query!("
                SELECT
                    manufacturing,
                    reaction
                FROM industry_index
                WHERE system_id = $1
                ORDER BY timestamp DESC
                LIMIT 1
            ",
                system,
            )
            .fetch_one(pool)
            .await
            .unwrap();

        system_index.insert(
            system.into(),
            (index.manufacturing, index.reaction)
        );
    }

    let material_cost = sqlx::query!("
                SELECT
                    type_id,
                    adjusted_price
                FROM market_prices
            ",
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|x| (x.type_id.into(), x.adjusted_price))
        .collect::<HashMap<_, _>>();

    let mut selected_tree: Option<EngineResult> = None;
    let mut selected_tree_total_cost = f32::INFINITY;

    for structure_group in structure_groups {
        let config = ProjectConfigBuilder::default()
            .add_blacklists(vec![4051, 4246, 4247, 4312])
            .add_blueprint_overwrites(blueprint_overwrites.clone())
            .add_structures(structure_group.structures)
            .add_structure_mappings(structure_group.mapping)
            .set_material_cost(material_cost.clone())
            .set_system_index(system_index.clone())
            .build();

        let mut engine = CalculationEngine::new(config);
        for dependency in project_data.products.iter() {
            let type_id = dependency.type_id;
            let json = sqlx::query!("
                    SELECT data
                    FROM   blueprint_json
                    WHERE  ptype_id = $1
                ",
                    *type_id
                )
                .fetch_one(pool)
                .await
                .map_err(|e| Error::FetchBlueprintJson(e, type_id))?
                .data;
            let json = serde_json::to_value(&json).unwrap();

            if let Ok(x) = Dependency::try_from(dependency.quantity as u32, json) {
                engine.add(x);
            } else {
                continue;
            };
        }

        let mut tree = engine;
        let tree = tree
            .apply_bonus()
            .add_stocks(&project_data.stocks)
            .finalize();
        tree.write_debug_file();

        let total_cost = tree.total_cost();

        if total_cost < selected_tree_total_cost {
            selected_tree = Some(tree);
            selected_tree_total_cost = total_cost;
        }
    }

    let (tree, stocks) = if let Some(x) = selected_tree {
        (x.tree, x.stocks)
    } else {
        return Ok(CostEstimateResponse {
            manufacturing_cost_total: 0f32,
            market_cost_total: 0f32,
            excess_cost_total: 0f32,

            excess_entries: Vec::new(),
        });
    };

    let mut excess_entries = Vec::new();
    tree
        .iter()
        .filter(|(_, x)| x.typ != BlueprintTyp::Material)
        .for_each(|(_, entry)| {
            let total_produced: u32 = entry
                .runs
                .iter()
                .map(|x| x * entry.produces as u32)
                .sum();

            let excess_quantity = total_produced.saturating_sub(entry.needed.ceil() as u32);

            if excess_quantity != 0 {
                excess_entries.push(AddExcess {
                    quantity: excess_quantity as i32,
                    type_id:  entry.product_type_id,
                });
            }
        });

    let mut stocks = stocks
        .into_iter()
        .filter(|x| x.quantity > 0)
        .collect::<Vec<_>>();
    stocks.sort_by_key(|x| x.type_id);
    stocks.dedup();

    let materials_required = tree
        .iter()
        .filter(|(_, x)| x.typ == BlueprintTyp::Material)
        .map(|(_, x)| {
            (
                x.product_type_id,
                x.needed.ceil() as u64,
            )
        })
        .collect::<HashMap<_, _>>();

    // Find markets that can support the required amount
    let mut viable_markets: HashMap<i32, MarketPrice> = HashMap::new();

    for (type_id, quantity) in materials_required {
        let prices = sqlx::query!(
            r#"
                SELECT
                    s.name AS "source",
                    mol.type_id,
                    remaining,
                    price
                FROM market_orders_latest mol
                JOIN structures s ON s.structure_id = mol.structure_id
                WHERE mol.type_id = $1
                AND is_buy = false
                ORDER BY price ASC
            "#,
                *type_id
            )
            .fetch_all(pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x|
                MarketPrice {
                    source:    x.source,
                    type_id:   x.type_id,
                    remaining: x.remaining as u64,
                    price:     x.price,
                    quantity:  quantity,
                }
            )
            .collect::<Vec<_>>();

        // Group all prices by the structure_id and type_id
        let mut grouped_by_station = HashMap::new();
        for price in prices {
            grouped_by_station
                .entry((price.source.clone(), price.type_id))
                .and_modify(|x: &mut Vec<MarketPrice>| x.push(price.clone()))
                .or_insert(vec![price.clone()]);
        }

        // Sort the vectors by price
        for (_, entries) in grouped_by_station.iter_mut() {
            entries.sort_by_key(|x| x.price.floor() as u64);
        }

        for ((_, type_id), entries) in grouped_by_station {
            let mut selected = MarketPrice::default();

            for entry in entries {
                if selected.quantity == 0 {
                    selected = entry;
                    continue;
                }

                // If there are more remaining entries than the quantity we need,
                // we found a viable market
                if selected.remaining >= selected.quantity {
                    if let Some(x) = viable_markets.get(&type_id) {
                        if selected.price < x.price {
                            viable_markets.insert(type_id, selected.clone());
                        }
                    } else {
                        viable_markets.insert(type_id, selected.clone());
                    }
                    break;
                }

                selected.remaining += entry.remaining;

                // If the price from the current entry is higher than the old price,
                // set the new value
                if selected.price < entry.price {
                    selected.price = entry.price;
                }
            }

            // The market does not have enough to support our needs
            if selected.remaining < selected.quantity {
                continue;
            }
        }
    }

    let manufacturing_cost_total: f32 = tree
        .iter()
        .filter(|(_, x)|
            x.typ == BlueprintTyp::Blueprint ||
            x.typ == BlueprintTyp::Reaction
        )
        .map(|(_, x)| x.build_cost.total_job_cost)
        .sum();

    let market_cost_total: f32 = viable_markets
        .iter()
        .map(|(_, x)| x.price as f32 * x.quantity as f32)
        .sum();

    let excess_cost_entries = excess(
            pool,
            excess_entries.clone(),
        )
        .await;
    let excess_cost_total: f32 = excess_cost_entries
        .clone()
        .into_iter()
        .map(|(_, x)| x)
        .sum();
    let excess_entries = excess_entries
        .into_iter()
        .map(|x| ExcessCostEstimateEntry {
            cost: *(excess_cost_entries.get(&x.type_id).unwrap_or(&0f32)),
            quantity: x.quantity,
            type_id: x.type_id
        })
        .collect::<Vec<_>>();

    Ok(CostEstimateResponse {
        manufacturing_cost_total,
        market_cost_total,
        excess_cost_total,

        excess_entries,
    })
}

async fn excess(
    pool:   &PgPool,
    excess: Vec<AddExcess>
) -> HashMap<TypeId, f32> {
    let entries = excess
        .into_iter()
        .map(|x| {
            AppraisalEntry {
                name:     String::new(),
                type_id:  x.type_id,
                quantity: x.quantity,
            }
        })
        .collect::<Vec<_>>();

    appraisal(
        &pool,
        AppraisalList::Internal,
        entries
    )
    .await
    .unwrap()
}
