use sqlx::PgPool;
use starfoundry_lib_projects::{BlueprintTyp, DependencyTreeEntry, MarketPrice};
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

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
) -> Result<HashMap<i32, MarketPrice>, Box<dyn std::error::Error>> {
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

    let mut viable_markets: HashMap<i32, MarketPrice> = HashMap::new();

    for (type_id, quantity) in materials_required {
        let prices = sqlx::query!(
            r#"
                SELECT
                    type_id,
                    remaining,
                    price
                FROM market_order_latest
                WHERE type_id = $1
                AND structure_id = ANY($2)
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
                MarketPrice {
                    source:    String::new(),
                    type_id:   x.type_id,
                    remaining: x.remaining as u64,
                    price:     x.price,
                    quantity:  quantity,
                }
            )
            .collect::<Vec<_>>();

        // Group all prices by the station_id and type_id
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

    Ok(viable_markets)
}
