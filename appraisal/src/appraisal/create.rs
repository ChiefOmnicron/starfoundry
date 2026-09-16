use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_appraisal::{Appraisal, AppraisalCode, AppraisalMode, AppraisalTotal, CreateAppraisalRequest};
use starfoundry_lib_eve_gateway::{EveGatewayApiClientItem, EveGatewayClient};
use starfoundry_lib_market::{MarketApiClientOrder, MarketBulkRequest, MarketClient, MarketItem, MarketStrategy};
use chrono::Utc;

use crate::api_docs::{BadRequest, InternalServerError, UnsupportedMediaType};
use crate::appraisal::{AppraisalError, Result};
use crate::{AppState, SERVICE_NAME};

/// Create Appraisal
/// 
/// - Alternative route: `/latest/appraisals`
/// - Alternative route: `/v1/appraisals`
/// 
/// ---
/// 
/// Creates a new appraisal
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "appraisal",
    request_body = CreateAppraisalRequest,
    responses(
        (
            body = Appraisal,
            description = "Newly created appraisal",
            status = CREATED,
        ),
        BadRequest,
        UnsupportedMediaType,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):           State<AppState>,
    Json(appraisal_info):   Json<CreateAppraisalRequest>,
) -> Result<impl IntoResponse> {
    appraisal_info.validate()?;

    let result = create(
            &state.postgres,
            appraisal_info,
        ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(result)
        )
    )
}

async fn create(
    pool:           &PgPool,
    appraisal_info: CreateAppraisalRequest,
) -> Result<Appraisal> {
    let code = AppraisalCode::new();
    let strategy = match appraisal_info.mode {
        AppraisalMode::Appraisal    => MarketStrategy::Appraisal,
        AppraisalMode::Multibuy     => MarketStrategy::MultiBuy,
        _                           => unimplemented!("Invalid appraisal mode: {:?}", appraisal_info.mode),
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

    // attempt to insert the information into the database. If it fails, it fails
    // silently
    if let Ok(market_entries) = serde_json::to_value(&entries) {
        let mode: String = appraisal_info.mode.clone().into();
        let result = sqlx::query!("
                INSERT INTO appraisal(
                    code,
                    market_id,
                    price_modifier,
                    market_info,
                    invalid_items,
                    comment,
                    raw,
                    mode
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ",
                &*code,
                *appraisal_info.market_id,
                appraisal_info.modifier as i16,
                market_entries,
                invalid_items.join("\n"),
                appraisal_info.comment,
                raw_content,
                &mode,
            )
            .execute(pool)
            .await;

        if let Err(e) = result {
            tracing::error!("{e}");
        }
    }

    let appraisal = Appraisal {
        code:           code,
        created_at_ts:  Utc::now().to_utc().timestamp_millis(),

        invalid:        invalid_items,
        items:          entries,
        total:          total,

        market_id:      appraisal_info.market_id,

        mode:           appraisal_info.mode,

        modifier:       appraisal_info.modifier,
        comment:        appraisal_info.comment,
        raw:            raw_content,
    };

    Ok(appraisal)
}
