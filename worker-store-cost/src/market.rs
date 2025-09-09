use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

use crate::engine::{BlueprintTyp, ViableMarketPrice, DependencyTreeEntry};

pub async fn material_cost(
    pool: &PgPool,
) -> Result<HashMap<TypeId, f64>, Box<dyn std::error::Error>> {
    let material_cost = sqlx::query!("
                SELECT
                    type_id,
                    adjusted_price
                FROM market_price
            ",
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|x| (x.type_id.into(), x.adjusted_price))
        .collect::<HashMap<_, _>>();
    Ok(material_cost)
}

pub async fn viable_markets(
    pool: &PgPool,
    tree: HashMap<TypeId, DependencyTreeEntry>,
    markets: Vec<i64>,
) -> Result<HashMap<i32, ViableMarketPrice>, Box<dyn std::error::Error>> {
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

    let mut viable_markets: HashMap<i32, ViableMarketPrice> = HashMap::new();

    for (type_id, quantity) in materials_required {
        let prices = sqlx::query!(
            r#"
                SELECT
                    s.name AS "source",
                    mol.type_id,
                    remaining,
                    price
                FROM market_order_latest mol
                JOIN structure s ON s.structure_id = mol.structure_id
                WHERE mol.type_id = $1
                AND mol.structure_id = ANY($2)
                AND is_buy = false
                ORDER BY price ASC
            "#,
                *type_id,
                &markets,
            )
            .fetch_all(pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x|
                ViableMarketPrice {
                    source:          x.source,
                    type_id:         x.type_id,
                    remaining:       x.remaining as u64,
                    price:           x.price,
                    quantity:        quantity,
                    incomplete_data: false,
                }
            )
            .collect::<Vec<_>>();

        // Group all prices by the station_id and type_id
        let mut grouped_by_station = HashMap::new();
        for price in prices {
            grouped_by_station
                .entry((price.source.clone(), price.type_id))
                .and_modify(|x: &mut Vec<ViableMarketPrice>| x.push(price.clone()))
                .or_insert(vec![price.clone()]);
        }

        // Sort the vectors by price
        for (_, entries) in grouped_by_station.iter_mut() {
            entries.sort_by_key(|x| x.price.floor() as u64);
        }

        let mut previous_iterations = Vec::new();
        for ((_, type_id), entries) in grouped_by_station {
            let mut selected = ViableMarketPrice::default();

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
                    //break;
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
                previous_iterations.push(selected);
                continue;
            }

            if selected.remaining >= selected.quantity {
                if let Some(x) = viable_markets.get(&type_id) {
                    if selected.price < x.price {
                        viable_markets.insert(type_id, selected.clone());
                    }
                } else {
                    viable_markets.insert(type_id, selected.clone());
                }
                continue;
            }
        }

        // no market had enough materials to fulfil the request
        // take the most expensive one
        if !viable_markets.contains_key(&type_id) {
            let mut solution = ViableMarketPrice::default();

            for market in previous_iterations {
                if (market.remaining as f64 * market.price) > (solution.remaining as f64 * solution.price) {

                    solution = market;
                    solution.incomplete_data = true;
                }
            }

            viable_markets.insert(*type_id, solution);
        }
    }

    Ok(viable_markets)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
    use std::collections::HashMap;
    use starfoundry_lib_items::Item;

    use crate::engine::{BlueprintTyp, DependencyBuildCost, DependencyTreeEntry};
    use crate::market::viable_markets;

    #[sqlx::test(fixtures("market_one"))]
    async fn happy_path(
        pool: PgPool,
    ) {
        let dummy_entry = DependencyTreeEntry {
            blueprint_type_id:  TypeId(34),
            product_type_id:    TypeId(34),
            needed:             250f32,
            time:               0f32,
            produces:           1i32,
            runs:               Vec::new(),
            children:           HashMap::new(),
            children_unbonused: HashMap::new(),
            typ:                BlueprintTyp::Material,
            item:               Item {
                name:          String::from("Tritanium"),
                volume:        0f32,

                category_id:   CategoryId(1),
                group_id:      GroupId(1),
                type_id:       TypeId(34),
                meta_group_id: None,
                repackaged:    None,
            },
            stock:              0i32,
            is_product:         false,
            build_cost:         DependencyBuildCost {
                total_job_gross:         0f32,
                material_cost_total:     0f32,
                facility:                0f32,
                scc:                     0f32,
                total_job_cost:          0f32,
                material_adjusted_price: HashMap::new(),
            },
            structure:          None,
            bonus:              Vec::new(),
        };

        let mut tree = HashMap::new();
        tree.insert(TypeId(34), dummy_entry);

        let markets = viable_markets(
                &pool,
                tree,
                vec![1],
            )
            .await
            .unwrap();

        assert_eq!(markets.len(), 1);
        assert_eq!(markets.get(&34).unwrap().quantity, 250);
        assert_eq!(markets.get(&34).unwrap().remaining, 500);
        assert_eq!(markets.get(&34).unwrap().price, 1f64);
        assert_eq!(markets.get(&34).unwrap().incomplete_data, false);
    }

    #[sqlx::test(fixtures("market_happy_path_2"))]
    async fn happy_path_2(
        pool: PgPool,
    ) {
        let dummy_entry = DependencyTreeEntry {
            blueprint_type_id:  TypeId(34),
            product_type_id:    TypeId(34),
            needed:             250f32,
            time:               0f32,
            produces:           1i32,
            runs:               Vec::new(),
            children:           HashMap::new(),
            children_unbonused: HashMap::new(),
            typ:                BlueprintTyp::Material,
            item:               Item {
                name:          String::from("Tritanium"),
                volume:        0f32,

                category_id:   CategoryId(1),
                group_id:      GroupId(1),
                type_id:       TypeId(34),
                meta_group_id: None,
                repackaged:    None,
            },
            stock:              0i32,
            is_product:         false,
            build_cost:         DependencyBuildCost {
                total_job_gross:         0f32,
                material_cost_total:     0f32,
                facility:                0f32,
                scc:                     0f32,
                total_job_cost:          0f32,
                material_adjusted_price: HashMap::new(),
            },
            structure:          None,
            bonus:              Vec::new(),
        };

        let mut tree = HashMap::new();
        tree.insert(TypeId(34), dummy_entry);

        let markets = viable_markets(
                &pool,
                tree,
                vec![1, 2],
            )
            .await
            .unwrap();

        assert_eq!(markets.len(), 1);
        assert_eq!(markets.get(&34).unwrap().quantity, 250);
        assert_eq!(markets.get(&34).unwrap().remaining, 1000);
        assert_eq!(markets.get(&34).unwrap().price, 0.9f64);
        assert_eq!(markets.get(&34).unwrap().incomplete_data, false);
    }

    #[sqlx::test(fixtures("market_happy_path_3"))]
    async fn happy_path_3(
        pool: PgPool,
    ) {
        let dummy_entry = DependencyTreeEntry {
            blueprint_type_id:  TypeId(34),
            product_type_id:    TypeId(34),
            needed:             250f32,
            time:               0f32,
            produces:           1i32,
            runs:               Vec::new(),
            children:           HashMap::new(),
            children_unbonused: HashMap::new(),
            typ:                BlueprintTyp::Material,
            item:               Item {
                name:          String::from("Tritanium"),
                volume:        0f32,

                category_id:   CategoryId(1),
                group_id:      GroupId(1),
                type_id:       TypeId(34),
                meta_group_id: None,
                repackaged:    None,
            },
            stock:              0i32,
            is_product:         false,
            build_cost:         DependencyBuildCost {
                total_job_gross:         0f32,
                material_cost_total:     0f32,
                facility:                0f32,
                scc:                     0f32,
                total_job_cost:          0f32,
                material_adjusted_price: HashMap::new(),
            },
            structure:          None,
            bonus:              Vec::new(),
        };

        let mut tree = HashMap::new();
        tree.insert(TypeId(34), dummy_entry);

        let markets = viable_markets(
                &pool,
                tree,
                vec![1, 2],
            )
            .await
            .unwrap();

        assert_eq!(markets.len(), 1);
        assert_eq!(markets.get(&34).unwrap().quantity, 250);
        assert_eq!(markets.get(&34).unwrap().remaining, 400);
        assert_eq!(markets.get(&34).unwrap().price, 1f64);
        assert_eq!(markets.get(&34).unwrap().incomplete_data, false);
    }

    #[sqlx::test(fixtures("market_two"))]
    async fn overflow_into_second_market(
        pool: PgPool,
    ) {
        let dummy_entry = DependencyTreeEntry {
            blueprint_type_id:  TypeId(34),
            product_type_id:    TypeId(34),
            needed:             750f32,
            time:               0f32,
            produces:           1i32,
            runs:               Vec::new(),
            children:           HashMap::new(),
            children_unbonused: HashMap::new(),
            typ:                BlueprintTyp::Material,
            item:               Item {
                name:          String::from("Tritanium"),
                volume:        0f32,

                category_id:   CategoryId(1),
                group_id:      GroupId(1),
                type_id:       TypeId(34),
                meta_group_id: None,
                repackaged:    None,
            },
            stock:              0i32,
            is_product:         false,
            build_cost:         DependencyBuildCost {
                total_job_gross:         0f32,
                material_cost_total:     0f32,
                facility:                0f32,
                scc:                     0f32,
                total_job_cost:          0f32,
                material_adjusted_price: HashMap::new(),
            },
            structure:          None,
            bonus:              Vec::new(),
        };

        let mut tree = HashMap::new();
        tree.insert(TypeId(34), dummy_entry);

        let markets = viable_markets(
                &pool,
                tree,
                vec![1, 2],
            )
            .await
            .unwrap();

        assert_eq!(markets.len(), 1);
        assert_eq!(markets.get(&34).unwrap().quantity, 750);
        assert_eq!(markets.get(&34).unwrap().remaining, 1000);
        assert_eq!(markets.get(&34).unwrap().price, 1.1f64);
        assert_eq!(markets.get(&34).unwrap().incomplete_data, false);
    }

    #[sqlx::test(fixtures("market_one"))]
    async fn more_than_the_market_can_handle(
        pool: PgPool,
    ) {
        let dummy_entry = DependencyTreeEntry {
            blueprint_type_id:  TypeId(34),
            product_type_id:    TypeId(34),
            needed:             1_000f32,
            time:               0f32,
            produces:           1i32,
            runs:               Vec::new(),
            children:           HashMap::new(),
            children_unbonused: HashMap::new(),
            typ:                BlueprintTyp::Material,
            item:               Item {
                name:          String::from("Tritanium"),
                volume:        0f32,

                category_id:   CategoryId(1),
                group_id:      GroupId(1),
                type_id:       TypeId(34),
                meta_group_id: None,
                repackaged:    None,
            },
            stock:              0i32,
            is_product:         false,
            build_cost:         DependencyBuildCost {
                total_job_gross:         0f32,
                material_cost_total:     0f32,
                facility:                0f32,
                scc:                     0f32,
                total_job_cost:          0f32,
                material_adjusted_price: HashMap::new(),
            },
            structure:          None,
            bonus:              Vec::new(),
        };

        let mut tree = HashMap::new();
        tree.insert(TypeId(34), dummy_entry);

        let markets = viable_markets(
                &pool,
                tree,
                vec![1],
            )
            .await
            .unwrap();

        assert_eq!(markets.len(), 1);
        assert_eq!(markets.get(&34).unwrap().quantity, 1000);
        assert_eq!(markets.get(&34).unwrap().remaining, 500);
        assert_eq!(markets.get(&34).unwrap().price, 1f64);
        assert_eq!(markets.get(&34).unwrap().incomplete_data, true);
    }

    #[sqlx::test(fixtures("market_two"))]
    async fn more_than_the_market_can_handle_multiple_markets(
        pool: PgPool,
    ) {
        let dummy_entry = DependencyTreeEntry {
            blueprint_type_id:  TypeId(34),
            product_type_id:    TypeId(34),
            needed:             1500f32,
            time:               0f32,
            produces:           1i32,
            runs:               Vec::new(),
            children:           HashMap::new(),
            children_unbonused: HashMap::new(),
            typ:                BlueprintTyp::Material,
            item:               Item {
                name:          String::from("Tritanium"),
                volume:        0f32,

                category_id:   CategoryId(1),
                group_id:      GroupId(1),
                type_id:       TypeId(34),
                meta_group_id: None,
                repackaged:    None,
            },
            stock:              0i32,
            is_product:         false,
            build_cost:         DependencyBuildCost {
                total_job_gross:         0f32,
                material_cost_total:     0f32,
                facility:                0f32,
                scc:                     0f32,
                total_job_cost:          0f32,
                material_adjusted_price: HashMap::new(),
            },
            structure:          None,
            bonus:              Vec::new(),
        };

        let mut tree = HashMap::new();
        tree.insert(TypeId(34), dummy_entry);

        let markets = viable_markets(
                &pool,
                tree,
                vec![1, 2],
            )
            .await
            .unwrap();

        assert_eq!(markets.len(), 1);
        assert_eq!(markets.get(&34).unwrap().quantity, 1500);
        assert_eq!(markets.get(&34).unwrap().remaining, 1000);
        assert_eq!(markets.get(&34).unwrap().price, 1.1f64);
        assert_eq!(markets.get(&34).unwrap().incomplete_data, true);
    }
}
