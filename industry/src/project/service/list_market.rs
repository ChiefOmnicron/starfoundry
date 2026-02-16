use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientItem, Item};
use starfoundry_lib_market::{BuyStrategy, MarketApiClientOrder, MarketBulkRequest, MarketBulkResponse, MarketItemList};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;
use crate::market_api_client;

pub async fn list_market(
    pool:                   &PgPool,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClientItem,
) -> Result<Vec<ProjectMarketGroup>> {
    let entries = sqlx::query!(r#"
            SELECT
                id,
                cost,
                type_id,
                quantity,
                source
            FROM project_market
            WHERE project_id = $1
        "#,
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::ListMisc)?;

    let mut type_ids = entries
        .iter()
        .map(|x| x.type_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    type_ids.push(62402.into());
    type_ids.sort();
    type_ids.dedup();

    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let tmp_market_orders_multi = market_api_client()
        .unwrap()
        .bulk_latest_orders(MarketBulkRequest {
            item_list: Some(
                entries.iter().map(|x| MarketItemList {
                    quantity: x.quantity,
                    type_id: x.type_id.into(),
                })
                .collect::<Vec<_>>()
            ),
            markets: vec![1046664001931i64, 1049588174021i64, 60003760i64, 60008494i64].into_iter().map(Into::into).collect::<Vec<_>>(),
            //markets: vec![1046664001931i64, 60003760i64, 60008494i64].into_iter().map(Into::into).collect::<Vec<_>>(),
            strategy: BuyStrategy::MultiBuy,
            ..Default::default()
        })
        .await
        .unwrap();
    let tmp_market_orders_smart = market_api_client()
        .unwrap()
        .bulk_latest_orders(MarketBulkRequest {
            item_list: Some(
                entries.iter().map(|x| MarketItemList {
                    quantity: x.quantity,
                    type_id: x.type_id.into(),
                })
                .collect::<Vec<_>>()
            ),
            markets: vec![1046664001931i64, 1049588174021i64, 60003760i64, 60008494i64].into_iter().map(Into::into).collect::<Vec<_>>(),
            //markets: vec![1046664001931i64, 60003760i64, 60008494i64].into_iter().map(Into::into).collect::<Vec<_>>(),
            strategy: BuyStrategy::SmartBuy,
            ..Default::default()
        })
        .await
        .unwrap();

    let mut project_market = Vec::new();
    for entry in entries {
        let item = if entry.type_id == 30375 {
            if let Some(x) = tmp_market_orders_smart.iter().find(|x| x.type_id == 62402) {
                items.get(&62402.into()).unwrap()
            } else {
                items.get(&entry.type_id.into()).unwrap()
            }
        } else {
            if let Some(x) = items.get(&entry.type_id.into()) {
                x
            } else {
                continue;
            }
        };

        let tmp_market_orders_multi = if let Some(entry) = tmp_market_orders_multi.iter().find(|x| x.type_id == entry.type_id) {
            entry.clone()
        } else {
            MarketBulkResponse { source: 0i64.into(), type_id: 0, quantity: 0, price: 0f64, insufficient_data: true }
        };
        let tmp_market_orders_smart = tmp_market_orders_smart
            .iter()
            .cloned()
            .filter(|x| {
                if entry.type_id == 30375 {
                    if x.type_id == 30375 || x.type_id == 62402 {
                        true
                    } else {
                        false
                    }
                } else {
                    x.type_id == entry.type_id
                }
            })
            .collect::<Vec<_>>();

        let project_group = ProjectMarket {
            id:         entry.id.into(),
            item:       item.clone(),
            quantity:   entry.quantity,

            cost:       entry.cost,
            source:     entry.source,

            cost_multi: tmp_market_orders_multi.clone(),
            cost_smart: tmp_market_orders_smart.clone(),
        };
        project_market.push(project_group);
    }

    Ok(sort_by_market_group(project_market))
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMarket {
    pub id:       Uuid,
    pub item:     Item,
    pub quantity: i32,

    pub cost:     Option<f64>,
    pub source:   Option<String>,

    pub cost_multi: MarketBulkResponse,
    pub cost_smart: Vec<MarketBulkResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMarketGroup {
    pub header:  String,
    pub entries: Vec<ProjectMarket>,
}

fn sort_by_market_group(entries: Vec<ProjectMarket>) -> Vec<ProjectMarketGroup> {
    let mut market_groups   = Vec::new();
    let mut grouped_entries = std::collections::HashMap::new();

    let mut insert_into_map = |id: i32, entry: ProjectMarket| {
        grouped_entries
            .entry(id)
            .and_modify(|x: &mut Vec<ProjectMarket>| x.push(entry.clone()))
            .or_insert(vec![entry]);
    };

    // First go through all entries, and sort them into the map
    for entry in entries.into_iter() {
        match *entry.item.category.category_id {
            7i32 => {
                insert_into_map(7, entry);
                continue;
            },
            8i32 => {
                insert_into_map(8, entry);
                continue;
            },
            25i32 => {
                insert_into_map(25, entry);
                continue;
            },
            _  => {}
        }

        match *entry.item.group.group_id {
            18i32 => {
                insert_into_map(18, entry);
                continue;
            },
            303i32 => {
                insert_into_map(303, entry);
                continue;
            },
            334i32 => {
                insert_into_map(334, entry);
                continue;
            },
            423i32 => {
                insert_into_map(423, entry);
                continue;
            },
            428i32 => {
                insert_into_map(428, entry);
                continue;
            },
            429i32 => {
                insert_into_map(429, entry);
                continue;
            },
            427i32 => {
                insert_into_map(427, entry);
                continue;
            },
            526i32 => {
                insert_into_map(526, entry);
                continue;
            },
            711i32 => {
                insert_into_map(711, entry);
                continue;
            },
            712i32 => {
                insert_into_map(712, entry);
                continue;
            },
            754i32 => {
                insert_into_map(754, entry);
                continue;
            },
            974i32 => {
                insert_into_map(974, entry);
                continue;
            },
            1136i32 => {
                insert_into_map(1136, entry);
                continue;
            },
            1042i32 => {
                insert_into_map(1042, entry);
                continue;
            },
            1034i32 => {
                insert_into_map(1034, entry);
                continue;
            },
            1040i32 => {
                insert_into_map(1040, entry);
                continue;
            },
            1041i32 => {
                insert_into_map(1041, entry);
                continue;
            },
            1996i32 => {
                insert_into_map(1996, entry);
                continue;
            },
            4168i32 => {
                insert_into_map(4168, entry);
                continue;
            },
            _  => {
                insert_into_map(0, entry);
            }
        }
    }

    // Secondly give the groups a name, and sort the entries
    for (header, id) in vec![
        ("COMPRESSED_MINERALS",       25),
        ("MINERALS",                  18),
        ("MOON_MATERIALS",           427),
        ("COMPRESSED_GAS",          4168),
        ("GAS",                      711),
        ("FUEL_BLOCKS",             1136),
        ("INTERMEDIATE_COMPOSITE",   428),
        ("COMPOSITE",                429),
        ("HYBRID_POLYMERS",          974),
        ("PI_TIER_1",               1042),
        ("PI_TIER_2",               1034),
        ("PI_TIER_3",               1040),
        ("PI_TIER_4",               1041),
        ("COMMODITIES",              526),
        ("CONSTRUCTION_COMPONENTS",  334),
        ("SALVAGE",                  754),
        ("MODULES",                    7),
        ("CHARGES",                    8),
        ("BOOSTER",                  303),
        ("ICE",                      423),
        ("BIOCHEMICAL",              712),
        ("ABYSSAL_MATERIALS",       1996),
        ("UNGROUPED",                  0),
    ] {
        if let Some(x) = grouped_entries.get_mut(&id) {
            x.sort_by_key(|x| x.item.name.clone());
            market_groups.push(
                ProjectMarketGroup {
                    header:  header.into(),
                    entries: x.clone(),
                }
            );
        }
    }

    market_groups
}
