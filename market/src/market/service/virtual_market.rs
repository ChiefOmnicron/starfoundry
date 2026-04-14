use sqlx::PgPool;
use starfoundry_lib_market::MarketVirtualRequest;
use starfoundry_lib_types::OrderId;
use std::collections::HashMap;

use crate::market::error::Result;

pub async fn virtual_market(
    pool:       &PgPool,
    entries:    Vec<MarketVirtualRequest>,
) -> Result<()> {
    let type_ids = entries
        .iter()
        .map(|x| x.type_id)
        .collect::<Vec<_>>();
    let structure_ids = entries
        .iter()
        .map(|x| x.market)
        .collect::<Vec<_>>();

    let mut market_entries = HashMap::new();
    sqlx::query!("
            SELECT
                order_id,
                type_id,
                structure_id,
                virtual_remaining
            FROM market_order_latest mol
            WHERE mol.type_id = ANY($1)
            AND mol.structure_id = ANY($2)
            AND mol.is_buy = false
            AND virtual_remaining > 0
            ORDER BY mol.price ASC
        ",
            &type_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
            &structure_ids.iter().map(|x| **x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| {
            let entry = TmpMarketEntry {
                order_id:           x.order_id.into(),
                virtual_remaining:  x.virtual_remaining,
            };

            market_entries
                .entry((x.type_id, x.structure_id))
                .and_modify(|x: &mut Vec<TmpMarketEntry>| x.push(entry.clone()))
                .or_insert(vec![entry]);
        });

    let mut updates = Vec::new();
    for entry in entries {
        let market_entries = if let Some(x) = market_entries.get(&(*entry.type_id, *entry.market)) {
            x
        } else {
            continue;
        };

        let mut remaining = entry.quantity;
        for market_entry in market_entries {
            if remaining == 0 {
                break;
            }

            if market_entry.virtual_remaining < remaining {
                updates.push(Update {
                    order_id:   market_entry.order_id,
                    remaining:  0
                });
                remaining -= market_entry.virtual_remaining;
            } else if market_entry.virtual_remaining >= remaining {
                updates.push(Update {
                    order_id:   market_entry.order_id,
                    remaining:  market_entry.virtual_remaining - remaining,
                });
                remaining = 0;
            }
        }
    }

    sqlx::query!("
            UPDATE market_order_latest AS mol
            SET virtual_remaining = update.remaining
            FROM UNNEST(
                $1::BIGINT[],
                $2::INTEGER[]
            ) as update(order_id, remaining)
            WHERE mol.order_id = update.order_id
        ",
            &updates.iter().map(|x| *x.order_id).collect::<Vec<_>>(),
            &updates.iter().map(|x| x.remaining).collect::<Vec<_>>(),
        )
        .execute(pool)
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone)]
struct TmpMarketEntry {
    order_id:           OrderId,
    virtual_remaining:  i32,
}

struct Update {
    order_id:   OrderId,
    remaining:  i32,
}
