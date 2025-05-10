use sqlx::PgPool;
use starfoundry_libs_structures::StructureUuid;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{Error, MarketRecommendation, ProjectUuid, Result};

pub async fn fetch_minerals(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Vec<MarketRecommendation>> {
    let prices = sqlx::query!(
        r#"
            SELECT
                s.name AS "source",
                pm.type_id,
                i.name AS item_name,
                i.volume,
                pm.quantity,
                remaining,
                price
            FROM market_order_latest mol
            JOIN project_market pm
              ON pm.type_id = mol.type_id
            JOIN item i
              ON i.type_id = pm.type_id
            JOIN structure s
              ON s.structure_id = mol.structure_id
            WHERE pm.source IS NULL
              AND pm.cost IS NULL
              AND project_id = $1
              -- Minerals
              AND group_id = 18
              -- Exclude Mercoxit
              AND i.type_id != 11399
              AND is_buy = false
            ORDER BY price ASC
        "#,
            *project_uuid,
        )
        .fetch_all(pool)
        .await 
        .map_err(|e| Error::FetchMarketPrices(e, project_uuid))?
        .into_iter()
        .map(|x|
            MarketRecommendation {
                item_name:    x.item_name,
                source:       x.source,
                // FIXME: when this code is touched again, should return the structure uuid
                structure_id: StructureUuid::new(Uuid::new_v4()),
                type_id:      x.type_id,
                remaining:    x.remaining as u64,
                price:        x.price,
                volume:       x.volume,
                quantity:     x.quantity as u64,
            }
        )
        .collect::<Vec<_>>();

    // Group all prices by the structure_id and type_id
    let mut grouped_by_station = HashMap::new();
    for price in prices {
        grouped_by_station
            .entry((price.source.clone(), price.type_id))
            .and_modify(|x: &mut Vec<MarketRecommendation>| x.push(price.clone()))
            .or_insert(vec![price]);
    }

    // Sort the vectors by price
    for (_, entries) in grouped_by_station.iter_mut() {
        entries.sort_by_key(|x| x.price.floor() as u64);
    }

    // Find markets that can support the required amount
    let mut viable_markets: HashMap<i32, MarketRecommendation> = HashMap::new();
    for ((_, type_id), entries) in grouped_by_station {
        let mut selected = MarketRecommendation::default();

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

    let markets = viable_markets
        .into_values()
        .collect::<Vec<_>>();
    Ok(markets)
}
