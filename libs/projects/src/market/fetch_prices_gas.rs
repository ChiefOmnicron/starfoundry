use sqlx::PgPool;
use starfoundry_libs_structures::StructureUuid;
use starfoundry_libs_types::TypeId;
use std::collections::{HashMap, BTreeMap};
use uuid::Uuid;

use crate::{Error, MarketRecommendation, ProjectUuid, Result};

pub async fn fetch_gas(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Vec<MarketRecommendation>> {
    let gases = sqlx::query_as!(
        RawGases,
        r#"
            SELECT
                pm.type_id,
                pm.quantity
            FROM project_market pm
            JOIN items i ON i.type_id = pm.type_id
            WHERE source IS NULL
              AND cost IS NULL
              AND project_id = $1
              -- Gases
              AND group_id = 711
        "#,
            *project_uuid,
        )
        .fetch_all(pool)
        .await 
        .map_err(|e| Error::FetchMarketPrices(e, project_uuid))?;

    let mut mapped = BTreeMap::new();
    uncompressed_compressed(gases)
        .into_iter()
        .for_each(|(a, b)| {
            // Uncompressed
            mapped.insert(*a.type_id, a.quantity);
            // Compressed
            mapped.insert(*b.type_id, b.quantity);
        });

    struct MarketEntry {
        pub source:    String,
        pub type_id:   i32,
        pub item_name: String,
        pub remaining: i32,
        pub volume:    f32,
        pub price:     f64,
    }
    let type_ids = mapped
        .keys()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();

    let prices = sqlx::query_as!(
        MarketEntry,
        r#"
            SELECT
                s.name AS "source",
                i.type_id,
                i.name AS item_name,
                i.volume,
                remaining,
                price
            FROM market_orders_latest mol
            JOIN items i ON i.type_id = mol.type_id
            JOIN structures s ON s.structure_id = mol.structure_id
            WHERE i.type_id = ANY($1::INTEGER[])
            AND is_buy = false
              ORDER BY price ASC
        "#,
            &type_ids as _,
        )
        .fetch_all(pool)
        .await 
        .map_err(|e| Error::FetchMarketPrices(e, project_uuid))?
        .into_iter()
        .map(|x| {
            MarketRecommendation {
                item_name:    x.item_name,
                source:       x.source,
                // FIXME: when this code is touched again, should return the structure uuid
                structure_id: StructureUuid::new(Uuid::new_v4()),
                type_id:      x.type_id,
                remaining:    x.remaining as u64,
                price:        x.price,
                volume:       x.volume,
                quantity:     mapped.get(&x.type_id).unwrap_or(&0).clone() as u64,
            }
        })
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
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let uncompressed_or_compressed = vec![
        (30375, 62402), (30376, 62404), (30370, 62399), (30371, 62397),
        (30372, 62398), (30373, 62403), (30374, 62400), (30377, 62406),
        (30378, 62405), (25268, 62396), (25279, 62386), (25275, 62387),
        (25273, 62390), (25277, 62391), (25276, 62392), (25278, 62393),
        (25274, 62394), (28694, 62377), (28695, 62379), (28696, 62380),
        (28697, 62381), (28698, 62382), (28699, 62383), (28700, 62384),
        (28701, 62385),
    ];

    let mut market_entries = Vec::new();
    for (uncompressed, compressed) in uncompressed_or_compressed {
        let uncompressed = if let Some(x) = markets.get(&uncompressed) {
            x.clone()
        } else {
            MarketRecommendation {
                item_name: format!("{}", compressed),
                price: f64::MAX,
                ..Default::default()
            }
        };

        let compressed = if let Some(x) = markets.get(&compressed) {
            x.clone()
        } else {
            MarketRecommendation {
                item_name: format!("{}", compressed),
                price: f64::MAX,
                ..Default::default()
            }
        };

        if uncompressed.price < compressed.price &&
            uncompressed.quantity > 0 {

            market_entries.push(uncompressed.clone());
        } else if compressed.quantity > 0 {
            market_entries.push(compressed.clone());
        }
    }

    Ok(market_entries)
}

fn uncompressed_compressed(
    gases: Vec<RawGases>,
) -> Vec<(RawGases, RawGases)> {
    let always_compress = true;

    let mut uncompressed_compressed = Vec::new();
    for gas in gases {
        // 95% is the max reprocessing efficiency
        // We need to mae up for that, so we add 5.264% additionally
        let compressed_count = (
            gas.quantity as f32 + (gas.quantity as f32 * 0.05264)
        ).ceil() as i32;

        let mut gas = gas;

        if always_compress {
            gas.quantity = compressed_count;
        }

        match *gas.type_id {
            // Fullerite-C28
            30375 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62402), compressed_count))
            ),
            // Fullerite-C32
            30376 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62404), compressed_count))
            ),
            // Fullerite-C50
            30370 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62399), compressed_count))
            ),
            // Fullerite-C60
            30371 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62397), compressed_count))
            ),
            // Fullerite-C70
            30372 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62398), compressed_count))
            ),
            // Fullerite-C72
            30373 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62403), compressed_count))
            ),
            // Fullerite-C84
            30374 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62400), compressed_count))
            ),
            // Fullerite-C320
            30377 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62406), compressed_count))
            ),
            // Fullerite-C540
            30378 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62405), compressed_count))
            ),
            // Amber Cytoserocin
            25268 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62396), compressed_count))
            ),
            // Azure Cytoserocin
            25279 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62386), compressed_count))
            ),
            // Celadon Cytoserocin
            25275 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62387), compressed_count))
            ),
            // Golden Cytoserocin
            25273 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62390), compressed_count))
            ),
            // Lime Cytoserocin
            25277 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62391), compressed_count))
            ),
            // Malalchite Cytoserocin
            25276 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62392), compressed_count))
            ),
            // Vermillion Cytoserocin
            25278 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62393), compressed_count))
            ),
            // Viridian Cytoserocin
            25274 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62394), compressed_count))
            ),
            // Amber Mykoserocin
            28694 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62377), compressed_count))
            ),
            // Azure Mykoserocin
            28695 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62379), compressed_count))
            ),
            // Celadon Mykoserocin
            28696 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62380), compressed_count))
            ),
            // Golden Mykoserocin
            28697 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62381), compressed_count))
            ),
            // Lime Mykoserocin
            28698 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62382), compressed_count))
            ),
            // Malachite Mykoserocin
            28699 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62383), compressed_count))
            ),
            // Vermillion Mykoserocin
            28700 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62384), compressed_count))
            ),
            // Viridian Mykoserocin
            28701 => uncompressed_compressed.push(
                (gas.clone(), RawGases::new(TypeId(62385), compressed_count))
            ),
            _ => continue
        };
    };
    uncompressed_compressed
}

#[derive(Clone, Debug)]
struct RawGases {
    type_id:  TypeId,
    quantity: i32,
}

impl RawGases {
    pub fn new(
        type_id:  TypeId,
        quantity: i32,
    ) -> Self {
        Self {
            type_id,
            quantity,
        }
    }
}

