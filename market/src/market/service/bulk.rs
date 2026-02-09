use sqlx::PgPool;
use starfoundry_lib_market::{BuyStrategy, MarketBulkRequest, MarketBulkResponse};
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

use crate::lp::{MarketLpEntry, MarketProblem};
use crate::market::error::Result;
use crate::eve_gateway_api_client;
use starfoundry_lib_eve_gateway::EveGatewayApiClientItem;

pub async fn bulk(
    pool:    &PgPool,
    request: MarketBulkRequest,
) -> Result<Vec<MarketBulkResponse>> {
    let mut market_data: HashMap<TypeId, Vec<MarketLpEntry>> = HashMap::new();
    let market_items = if let Some(items) = request.item_list {
        items
    } else {
        return Ok(Vec::new());
    };

    let mut type_ids = market_items
        .iter()
        .map(|x| x.type_id)
        .collect::<Vec<_>>();
    type_ids.push(62402.into()); // Compressed Fullerite-C28

    let items = eve_gateway_api_client()
        .unwrap()
        .fetch_item_bulk(type_ids.clone())
        .await
        .unwrap()
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let market_prices = sqlx::query!("
            SELECT *
            FROM market_order_latest mol
            WHERE mol.type_id = ANY($1)
            AND mol.structure_id = ANY($2)
            AND mol.is_buy = false
            ORDER BY mol.price ASC
        ",
            &type_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
            &request.markets.iter().map(|x| **x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .unwrap();

    if request.strategy == BuyStrategy::SmartBuy {
        let start = std::time::Instant::now();
        let mut results: Vec<MarketBulkResponse> = Vec::new();

        market_prices
            .into_iter()
            .for_each(|x| {
                let entry = MarketLpEntry {
                    order_id:     x.order_id,
                    structure_id: x.structure_id,
                    price:        x.price,
                    quantity:     x.remaining,
                    item_volume:  items.get(&x.type_id.into()).unwrap().volume as f64,
                    type_id:      x.type_id.into(),
                };

                market_data
                    .entry(x.type_id.into())
                    .and_modify(|x: &mut Vec<MarketLpEntry>| x.push(entry.clone()))
                    .or_insert(vec![entry]);
            });

        for item in market_items {
            let mut data = market_data.get(&item.type_id).unwrap().clone();

            if item.type_id == TypeId(30375) {
                data.extend(market_data.get(&TypeId(62402)).unwrap().clone());
                data.sort_by(|a, b| a.price.total_cmp(&b.price));
            }

            let mut lp = MarketProblem::new();
            lp.calculate_market(data.clone());
            let result = lp
                .solve(item.quantity);

            let result = result
                .into_iter()
                .map(|(structure_id, x)| MarketBulkResponse {
                    insufficient_data: false,
                    price: x.price,
                    quantity: x.quantity as u64,
                    remaining: 0,
                    source: structure_id,
                    type_id: *x.type_id,
                })
                .collect::<Vec<_>>();
            results.extend(result);
        }
        dbg!("smart", start.elapsed().as_millis());
        return Ok(results);
    } else if request.strategy == BuyStrategy::MultiBuy {
        let start = std::time::Instant::now();
        let mut viable_markets: HashMap<i32, MarketBulkResponse> = HashMap::new();

        for item in market_items {
            // Group all prices by the station_id and type_id
            let mut grouped_by_station = HashMap::new();
            for price in market_prices.iter().filter(|x| x.type_id == *item.type_id) {
                let entry = MarketBulkResponse {
                    insufficient_data: false,
                    price: price.price,
                    quantity: item.quantity as u64,
                    remaining: price.remaining as u64,
                    source: price.structure_id,
                    type_id: *item.type_id,
                };

                grouped_by_station
                    .entry((price.structure_id, price.type_id))
                    .and_modify(|x: &mut Vec<MarketBulkResponse>| x.push(entry.clone()))
                    .or_insert(vec![entry.clone()]);
            }

            // Sort the vectors by price
            for (_, entries) in grouped_by_station.iter_mut() {
                entries.sort_by_key(|x| x.price.floor() as u64);
            }

            let mut previous_iterations = Vec::new();
            for ((_, type_id), entries) in grouped_by_station {
                let mut selected = MarketBulkResponse::default();

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
            if !viable_markets.contains_key(&*item.type_id) {
                let mut solution = MarketBulkResponse::default();

                for market in previous_iterations {
                    if (market.remaining as f64 * market.price) > (solution.remaining as f64 * solution.price) {

                        solution = market;
                        solution.insufficient_data = true;
                    }
                }

                viable_markets.insert(*item.type_id, solution);
            }
        }

        let result = viable_markets
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        dbg!("multi", start.elapsed().as_millis());
        return Ok(result);
    } else {
        return Ok(Vec::new());
    }
}

/*
#[cfg(test)]
mod list_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::project::service::list::ProjectFilter;
    use crate::test_util::EveGatewayTestApiClient;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let gateway_client = EveGatewayTestApiClient::new();
        let result = dbg!(super::list(
                &pool,
                CharacterId(1),
                ProjectFilter::default(),
                &gateway_client,
            )
            .await);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);

        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectFilter {
                    name: Some(String::from("Filter")),
                    ..Default::default()
                },
                &gateway_client,
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectFilter {
                    name: Some(String::from("SomeGibberish")),
                    ..Default::default()
                },
                &gateway_client,
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
*/
