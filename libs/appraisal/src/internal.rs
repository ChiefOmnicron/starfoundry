use item::InternalItem;
use response::InternalResponse;
use sqlx::PgPool;
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;

use crate::{ExternalAppraisal, AppraisalEntry, Error, Persistance, Result};

mod compression;
mod create;
mod fetch;
mod item;
mod models;
mod reprocessing;
mod response;

pub use self::compression::*;
pub use self::create::*;
pub use self::fetch::*;
pub use self::models::*;
pub use self::reprocessing::*;

pub struct InternalAppraisal(PgPool);

impl InternalAppraisal {
    pub fn new(
        pool: PgPool,
    ) -> Result<Self> {
        Ok(Self(pool))
    }
}

#[async_trait::async_trait]
impl ExternalAppraisal<InternalResponse> for InternalAppraisal
where
    Self: Sized,
{
    fn validate() -> Result<()> {
        Ok(())
    }

    async fn create(
        &self,
        _persist: Persistance,
        entries:  Vec<AppraisalEntry>,
    ) -> Result<InternalResponse> {
        let entries = entries
            .iter()
            .map(|x| (x.type_id, x.quantity))
            .collect::<HashMap<_, _>>();

        let type_ids = entries
            .keys()
            .map(|x| **x)
            .collect::<Vec<_>>();

        let jita_prices = sqlx::query!(
            r#"
                SELECT
                    i.type_id,
                    i.name AS item_name,
                    remaining,
                    price
                FROM market_orders_latest mol
                JOIN items i ON i.type_id = mol.type_id
                WHERE mol.type_id = ANY($1)
                AND mol.structure_id = 60003760
                AND is_buy = false
                ORDER BY price ASC
            "#,
                &type_ids
            )
            .fetch_all(&self.0)
            .await
            .map_err(Error::FetchInternalMarketPrices)?
            .into_iter()
            .map(|x|
                MarketPriceEntry {
                    item_name: x.item_name,
                    type_id:   x.type_id,
                    remaining: x.remaining,
                    price:     x.price,
                    quantity:  entries
                                    .get(&TypeId(x.type_id))
                                    .copied()
                                    .unwrap_or_default(),
                }
            )
            .collect::<Vec<_>>();

        let mut grouped = HashMap::new();
        jita_prices
            .into_iter()
            .for_each(|x| {
                grouped
                    .entry(x.type_id)
                    .and_modify(|y: &mut Vec<MarketPriceEntry>| y.push(x.clone()))
                    .or_insert(vec![x]);
            });

        let mut appraisal = Vec::new();
        for (type_id, quantity) in entries {
            let market_entries = grouped
                .get(&type_id)
                .cloned()
                .unwrap_or_default();

            let mut needed = quantity;

            for entry in market_entries {
                if needed <= entry.remaining {
                    appraisal.push(InternalItem {
                        sell: entry.price,
                        total_sell: entry.price * quantity as f64,
                        quantity,
                        type_id,
                    });
                    break;
                } else {
                    needed -= entry.remaining;
                    continue;
                }
            }
        }

        Ok(InternalResponse {
            items: appraisal,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct MarketPriceEntry {
    pub type_id:   i32,
    pub item_name: String,
    pub remaining: i32,
    pub price:     f64,
    pub quantity:  i32,
}
