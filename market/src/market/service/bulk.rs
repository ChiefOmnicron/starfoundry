use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

use crate::market::error::Result;
use crate::lp::{MarketLpEntry, MarketProblem};

pub async fn bulk(
    pool:    &PgPool,
    request: MarketBulkRequest,
) -> Result<Vec<String>> {
    if let Some(items) = request.item_list {
        // TODO: extract
        let mut market_data: HashMap<TypeId, Vec<MarketLpEntry>> = HashMap::new();
        sqlx::query!("
                SELECT *
                FROM market_order_latest
                WHERE type_id = ANY($1)
            ",
                &items.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
            )
            .fetch_all(pool)
            .await
            .unwrap()
            .into_iter()
            .for_each(|x| {
                let entry = MarketLpEntry {
                    order_id:     x.order_id,
                    structure_id: x.structure_id,
                    price:        x.price,
                    quantity:     x.remaining,
                };

                market_data
                    .entry(x.type_id.into())
                    .and_modify(|x: &mut Vec<MarketLpEntry>| x.push(entry.clone()))
                    .or_insert(vec![entry]);
            });

        for item in items {
            let data = market_data.get(&item.type_id).unwrap();

            let mut lp = MarketProblem::new();
            lp.calculate_market(data.clone());
            lp.solve(item.quantity);
        }
    }

    Ok(Vec::new())
}



/// Bulk request for resolving prices
/// 
/// Either `appraisal` or `item_list` must be set
/// 
#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
pub struct MarketBulkRequest {
    pub strategy: BuyStrategy,
    pub markets:  Vec<StructureId>,

    pub appraisal: Option<String>,
    pub item_list: Option<Vec<MarketItemList>>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct MarketItemList {
    pub type_id:  TypeId,
    pub quantity: i32,
}

/// Different strategies for buying materials
/// 
#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    Deserialize, Serialize, ToSchema,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuyStrategy {
    /// Acts like the in-game multi buy window
    /// 
    /// Advantages:
    /// - faster
    /// 
    /// Disadvantages:
    /// - can only buy from one market
    /// - no support for hauling costs
    /// - if a market does not have enough of the requested item type, it will
    ///   use the last price value
    MultiBuy,
    /// Looks at multiple markets in a detailed view
    /// 
    /// Advantages:
    /// - can buy from multiple markets
    /// - considers hauling costs
    /// 
    /// Disadvantages:
    /// - slower
    /// - depending on how old the market data is, the results may no longer be
    ///   valid
    SmartBuy,
}

impl Default for BuyStrategy {
    fn default() -> Self {
        Self::MultiBuy
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
