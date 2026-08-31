use chrono::Utc;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientItem, EveGatewayClient};
use starfoundry_lib_market::{MarketApiClientOrder, MarketBulkRequest, MarketClient, MarketItem, MarketStrategy};

use crate::appraisal::create::model::{Appraisal, AppraisalMode, AppraisalTotal, CreateAppraisalRequest};
use crate::appraisal::{AppraisalCode, AppraisalError, Result};
use crate::SERVICE_NAME;

pub async fn create(
    pool:           &PgPool,
    appraisal_info: CreateAppraisalRequest,
) -> Result<Appraisal> {
    let code = AppraisalCode::new();
    let strategy = match appraisal_info.mode {
        AppraisalMode::Appraisal    => MarketStrategy::Appraisal,
        AppraisalMode::Multibuy     => MarketStrategy::MultiBuy,
    };

    let raw_content = if let Some(x) = appraisal_info.item_str.as_ref() {
        Some(x.into())
    } else {
        None
    };

    let mut invalid_items = Vec::new();

    let eve_gateway_client = EveGatewayClient::new(SERVICE_NAME)?;
    let market_items = if let Some(items) = appraisal_info.item_list {
        items
    } else if let Some(x) = appraisal_info.item_str {
        let parsed = eve_gateway_client
            .parse_items(x)
            .await;

        if let Ok(x) = parsed {
            invalid_items = x.invalid;
            x.items
                .into_iter()
                .map(|x| MarketItem {
                    quantity:   x.quantity as i32,
                    type_id:    x.type_id,
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        return Err(AppraisalError::InvalidAppraisal("either `item_list` or `item_str` must be set".into()));
    };

    let market_client = MarketClient::new(SERVICE_NAME)?;
    let entries = market_client
        .bulk_latest_orders(MarketBulkRequest {
            item_list:          Some(market_items),
            markets:            vec![appraisal_info.market_id],
            strategy:           strategy,

            item_list_str:      None,
            smart_buy_config:   None,
            virtual_market:     false,
        })
        .await?;

    let total_buy = entries
        .iter()
        .filter(|x| x.buy_price.is_some())
        .map(|x| x.buy_price.as_ref().map(|y| y.max * x.quantity as f64).unwrap_or_default())
        .sum();
    let total_sell = entries
        .iter()
        .filter(|x| x.sell_price.is_some())
        .map(|x| x.sell_price.as_ref().map(|y| y.min * x.quantity as f64).unwrap_or_default())
        .sum();
    let total = AppraisalTotal {
        buy:    total_buy,
        sell:   total_sell,
    };

    let appraisal = Appraisal {
        code:           code,
        created_at_ts:  Utc::now().to_utc().timestamp_millis(),

        invalid:        invalid_items,
        items:          entries,
        total:          total,

        market_id:      appraisal_info.market_id,

        modifier:       appraisal_info.modifier,
        comment:        appraisal_info.comment,
        raw:            raw_content,
    };

    Ok(appraisal)
}
